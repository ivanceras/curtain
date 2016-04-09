
use iron::status;
use router::Router;
use std::str::FromStr;
use std::env;
use iron::prelude::*;
use persistent::{Write, State};
use std::net::SocketAddrV4;
use std::net::Ipv4Addr;
use global::GlobalPools;
use iron::method::Method::*;
use iron::AfterMiddleware;
use unicase::UniCase;
use iron::headers;
use inquerest;
use global;
use global::Context;
use window_service::window_api;
use validator::DbElementValidator;
use from_query::FromQuery;
use rustorm::query::Query;
use inquerest as iq;
use rustorm::dao::Dao;
use rustorm::dao::DaoResult;
use rustorm::table::Table;
use rustorm::query::Equality;
use rustorm::query::{Filter,Join, JoinType, Modifier, Condition, Operand, Connector};
use rustorm::query::{TableName,ToTableName};
use rustorm::query::source::ToSourceField;
use rustorm::query::column_name::{ToColumnName,ColumnName};
use rustc_serialize::json;

pub fn complex_query(context: &mut Context, main_table_filter: &TableFilter, rest_table_filter: &Vec<TableFilter>)->Result<RestData, ParseError>{
	let validator = DbElementValidator::from_context(context);
	let main_validated = main_table_filter.transform(context, &validator);
	match main_validated{
		Ok(main_validated) => {
			let mut rest_validated = vec![];
			for rest in rest_table_filter{
				match rest.transform(context, &validator){
					Ok(rtransformed) => {
						rest_validated.push(rtransformed);
					},
						Err(e) => ()
				}
			}
			let rest_data:Result<RestData, ParseError> = retrieve_main_data(context, &main_validated, &rest_validated);
			rest_data
		},
			Err(e) => Err(e) 
	}
}

/// retrieve the data on this table using the query
/// based on the focused dao, a filter will be added
/// for the rest of the table that has link to the main table
/// base on the filter of the main table and the primary key,
/// all other corresponding table will be left join to tho the main table
/// and included the filter of each other corresponding table.
/// main_query -> main_dao
/// main_query + table1_query -> table_dao

/// list of table data for each table names
#[derive(Debug)]
#[derive(RustcEncodable)]
pub struct RestData{
	table_dao: Vec<TableDao>,
}

impl RestData{
	
	fn empty()->Self{
		RestData{
			table_dao: vec![]
		}
	}
}


/// retrieve the window data for all tabs involved in this window
fn retrieve_main_data(context: &mut Context, main_query: &ValidatedQuery, rest_vquery: &Vec<ValidatedQuery>)->Result<RestData, ParseError>{
	let main_table:Table = main_query.table.clone();
	let mut table_dao = vec![];
	let mut mquery: Query = match main_query.query{
		Some(ref query) => query.clone(),
			None => Query::new()
	};
	println!("----->>> {:#?}", mquery);
	mquery.from(&main_table.clone());
	let main_dao_result = {
		let db = match context.db(){
			Ok(db) => db,
			Err(e) => {return Err(ParseError::new("unable to obtain db connection"));}
		};
		match mquery.retrieve(db){
			Ok(main_dao_result) => main_dao_result,
				Err(e) => {return Err(ParseError::new("unable to retrieve main dao results"));}
		}
	};
	table_dao.push(TableDao::from_dao_result(&main_dao_result, &main_table.complete_name()));
	let main_focused_dao = if main_dao_result.dao.len() > main_query.focused{
		&main_dao_result.dao[main_query.focused]
	}else if main_dao_result.dao.len() > 0 {
		warn!("focused index is out of bounds...");
		&main_dao_result.dao[0]
	}else{
		warn!("this table is empty");
		return Ok(RestData::empty());//returning empty result early
	};
	let main_focused_filter = create_main_query_join_filter_from_focused_dao(&main_table, &main_focused_dao);
	let main_filter:Vec<Filter> = extract_comprehensive_filter(&main_table, &mquery);
	let mut main_with_focused_filter = main_filter.clone();
	main_with_focused_filter.extend_from_slice(&main_focused_filter);
	let main_window = match window_api::retrieve_window(context, &main_table.name){
		Ok(main_window) => main_window,
			Err(e) => {return Err(ParseError::new("unable to obtain main window"));}
	};
	if let Some(main_tab) = main_window.tab{
		if let &Some(ref ext_tabs) = &main_tab.ext_tabs{
			for ext_tab in ext_tabs{
				let ext_table = match window_api::get_matching_table(context, &ext_tab.table){
					Some(ext_table) => ext_table,
						None => {return Err(ParseError::new("Unable to get table for extension"));}
				};
				let mut ext_query = build_query(&main_table, &ext_table, &main_with_focused_filter, rest_vquery);
				let debug = ext_query.debug_build(context.db().unwrap());
				println!("debug sql: {}", debug);
				match ext_query.retrieve(context.db().unwrap()){
					Ok(dao_result) => {
						table_dao.push(TableDao::from_dao_result(&dao_result, &ext_table.complete_name()));
					},
						Err(e) => {
							return Err(ParseError::new("unable to retrove data in main table"));
						}
				}
			}
		}
		if let &Some(ref has_many_tabs) = &main_tab.has_many_tabs{
			for has_many in has_many_tabs{
				let has_table = match window_api::get_matching_table(context, &has_many.table){
					Some(has_table) => has_table,
						None => {return Err(ParseError::new("Unable to get table for extension"));}
				};
				let mut has_query = build_query(&main_table, &has_table, &main_with_focused_filter, rest_vquery);
				let debug = has_query.debug_build(context.db().unwrap());
				println!("debug sql: {}", debug);
				match has_query.retrieve(context.db().unwrap()){
					Ok(dao_result) => {
						table_dao.push(TableDao::from_dao_result(&dao_result, &has_table.complete_name()));
					}
					Err(e) => {
						return Err(ParseError::new("unable to retrieve data in has many table"));
					}
				}
			}
		}
		if let &Some(ref has_many_indirect_tabs) = &main_tab.has_many_indirect_tabs{
			for indirect in has_many_indirect_tabs{
				// will use inner join to linker table, and inner join to the indirect
				//println!("indirect: {:#?}", indirect);
				let indirect_table = match window_api::get_matching_table(context, &indirect.table){
					Some(indirect_table) => indirect_table,
						None => {return Err(ParseError::new("Unable to get table for extension"));}
				};
				assert!(indirect.linker_table.is_some(), format!("indirect tab {} must have a linker table specified", indirect.table));
				let linker_table_name = &indirect.linker_table.as_ref().unwrap();
				let linker_table =match window_api::get_matching_table(context, &linker_table_name){
					Some(linker_table) => linker_table,
						None => { return Err(ParseError::new("linker table can not be found")); }
				};
				//println!("linker table of {}: {:#?}", indirect.table, linker_table);
				let mut ind_query = build_query_with_linker(&main_table, &indirect_table, &main_with_focused_filter, &linker_table, rest_vquery);

				let debug = ind_query.debug_build(context.db().unwrap());
				println!("indirect query debug sql: {}", debug);
				match ind_query.retrieve(context.db().unwrap()){
					Ok(dao_result) => {
						table_dao.push(TableDao::from_dao_result(&dao_result, &indirect_table.complete_name()));
					}
					Err(e) => {
						return Err(ParseError::new("Unable to retrieve data from indirect table"));
					}
				}

			}
		}
		let rest_data = RestData{
			table_dao: table_dao
		};
		Ok(rest_data)
	}else{
		Err(ParseError::new("no main tab"))
	}
}


fn build_query(main_table: &Table, ext_table: &Table, main_filter: &Vec<Filter>, rest_vquery: &Vec<ValidatedQuery>)->Query{
	let mut ext_query = Query::select();
	let ext_table_name = ext_table.to_table_name();
	ext_query.enumerate_from_table(&ext_table_name);
	ext_query.from(main_table);
	let on_filter = create_on_filter(&main_table, &ext_table);
	let ext_join = Join{
		modifier: None,
		join_type: Some(JoinType::INNER),
		table_name: ext_table_name,
		on: on_filter
	};
	ext_query.joins.push(ext_join);
	for filter in main_filter{
		ext_query.add_filter(filter.clone());
	}
	let ext_filters = rest_vquery.find_query(ext_table);
	if let Some(ext_filters) = ext_filters{
		for ext_filter in ext_filters.filters{
			ext_query.add_filter(ext_filter);
		}
	}
	ext_query
}

fn build_query_with_linker(main_table: &Table, indirect_table: &Table, main_filter: &Vec<Filter>, linker_table: &Table, rest_vquery: &Vec<ValidatedQuery>)->Query{
	let mut query = Query::select();
	query.enumerate_from_table(&indirect_table.to_table_name());
	query.from(main_table);
	let join1 = Join{
		modifier: None,
		join_type: Some(JoinType::INNER),
		table_name: linker_table.to_table_name(),
		on: create_on_filter(&main_table, &linker_table)
	};
	query.joins.push(join1);
	let join2 = Join{
		modifier: None,
		join_type: Some(JoinType::INNER),
		table_name: indirect_table.to_table_name(),
		on: create_on_filter(&indirect_table, &linker_table)
	};
	query.joins.push(join2);
	for filter in main_filter{
		query.add_filter(filter.clone());
	}
	let ind_query = rest_vquery.find_query(indirect_table);
	if let Some(ind_query) = ind_query{
		for indirect_filter in ind_query.filters{
			query.add_filter(indirect_filter);
		}
	}
	query
}

/// extract the filters of this query from main table, but adds details to the column names to avoid unambigous feild
/// if there is no table specified in the column name, the main table is added, else leave it as it is
fn extract_comprehensive_filter(main_table: &Table, main_query: &Query)->Vec<Filter>{
	let mut filters = vec![];
	for filter in &main_query.filters{
		println!("comprehensive filter: {:?}", filter);
		let mut condition = filter.condition.clone();
		let left = match &condition.left{
			&Operand::ColumnName(ref column) => {
				let mut column = column.clone();
				if column.table.is_some(){
					//leave as is	
				}else{
					column.table = Some(main_table.name.to_owned())
				}
				println!("left column: {:?}", column);
				Operand::ColumnName(column)
			}
			_ => condition.left.clone()
		};
		let right = match &condition.right{
			&Operand::ColumnName(ref column) => {
				let mut column = column.clone();
				if column.table.is_some(){
					//leave as is	
				}else{
					column.table = Some(main_table.name.to_owned())
				}
				Operand::ColumnName(column)
			}
			_ => condition.right.clone()
		};
		condition.left = left;
		condition.right = right;
		let mut new_filter = filter.clone();
		new_filter.condition = condition;
		filters.push(new_filter);
	}
	filters
}

fn create_on_filter(main_table: &Table, table: &Table)->Filter{
	let foreign_columns = table.get_foreign_columns_to_table(&main_table);
	let mut filters = vec![];
	for fc in foreign_columns{
		println!("foreign column: {:#?}", fc);
		let fk = match fc.foreign{
			Some(ref fk) => format!("{}.{}",fk.table, fk.column),
			None => panic!("no foreign key found"),
		};
		let condition = Condition{
			left: Operand::ColumnName(fc.to_column_name()),
			equality: Equality::EQ,
			right: Operand::ColumnName(ColumnName::from_str(&fk)),
		};
		let filter = Filter{
			connector: Connector::And,
			condition: condition,
			sub_filters: vec![],
		};
		filters.push(filter);
	}

	// flatten filter into 1, by anding the rest to the first filter;
	assert!(filters.len() > 0, "There must be at least 1 filter created");
	let mut first_filter = filters[0].clone();
	if filters.len() > 1{
		let rest_filter = &filters[1..filters.len()];
		for rfilter in rest_filter{
			first_filter.sub_filters.push(rfilter.clone());
		}
	}
	println!("on filters: {:#?}", filters);
	println!("first filter: {:#?}", first_filter);
	first_filter
}

trait QuerySearch{
	fn find_query(&self, table: &Table)->Option<Query>;
}

impl QuerySearch for Vec<ValidatedQuery>{

	fn find_query(&self, table: &Table)->Option<Query>{
		for vquery in self{
			//find if the vquery will match this table
			if &vquery.table == table{
				return vquery.query.clone();
			}

		}
		None
	}	

}

/// retrieve data from table utilizing the main table as the first table with its associated filter and left joining the vquery
fn retrieve_data_from_table(context: &mut Context, vquery: &ValidatedQuery, main_table: &Table, main_filter: &Vec<Filter>){
	let mut query = Query::select();
	let table = vquery.table.to_owned();
	query.only_from(&table);
	//get the foreign key of table that points to the primary key of the main table
	println!("query: {:#?}", query);
	let foreign_columns = table.get_foreign_columns_to_table(&main_table);
	println!("foreign columns: {:#?}", foreign_columns);
}

fn create_main_query_join_filter_from_focused_dao(table: &Table, focused_dao: &Dao)->Vec<Filter>{
	let mut filters = vec![];
	for pri in table.primary_columns(){
		let pvalue = focused_dao.get_value(&pri.name);
		let column = pri.complete_name();
		let filter = Filter::new(&column, Equality::EQ, &pvalue);
		filters.push(filter)
	}
	filters
}

/// determines whether to update, insert the data
#[derive(Debug)]
#[derive(RustcEncodable)]
enum DataState{
	Orig, //original record, untouched
	Inserted,// a newly inserted record
	Dirty, // the record is edited and will be updated to the database
	Deleted, // record is deleted
}
#[derive(Debug)]
#[derive(RustcEncodable)]
struct DaoState{
	state: DataState,
	focused: bool,
	dao: Dao,
}

impl DaoState{

	fn from_dao(dao: Dao)->Self{
		DaoState{
			state: DataState::Orig,
			focused: false,
			dao: dao,
		}
	}
	
	fn from_dao_result(dao_result: &DaoResult)->Vec<Self>{
		let mut dao_states = vec![];
		for dao in &dao_result.dao{
			let ds = DaoState::from_dao(dao.clone());
			dao_states.push(ds)
		}
		dao_states
	}
}

#[derive(Debug)]
#[derive(RustcEncodable)]
struct TableDao{
	table: String,
	dao_list: Vec<DaoState>,
} 


impl TableDao{
	
	fn from_dao_result(dao_result: &DaoResult, table_name: &str)->Self{
		TableDao{
			table: table_name.to_owned(),
			dao_list: DaoState::from_dao_result(dao_result),
		}
	}
}



#[derive(Debug)]
struct ValidatedQuery{
	table: Table,
	query: Option<Query>,
	focused: usize,
}

#[derive(Debug)]
pub struct TableFilter{
	pub table: String,
	pub filter: Option<String>
}

impl TableFilter{

	pub fn transform(&self, context: &mut Context, validator: &DbElementValidator)->Result<ValidatedQuery, ParseError>{

		if validator.is_valid_table(&self.table){
			let table = match window_api::get_matching_table(context, &self.table){
				Some(table) => table,
				None => {return Err(ParseError::new("Unable to get matching table")); }
			};
			match self.filter{
				Some(ref filter) => {
					let parsed = inquerest::query(&filter);
					println!("parsed: {:#?}",parsed);
					match parsed{
						Ok(parsed) => {
							let focused = extract_focused(&parsed);
							println!("focused: {}", focused);
							let transformed = parsed.transform(&validator);
							let vquery = ValidatedQuery{
								table: table,
								query: Some(transformed),
								focused: focused,
							};
							Ok(vquery)
						},
						Err(e) => {
							Err(ParseError::new("unable to parse query"))
						}
					}
				},
				None => {
					let vquery = ValidatedQuery{
						table: table,
						query: None,
						focused: 0
					};
					Ok(vquery)
				}
			}
		}else{
			Err(ParseError::new("table is invalid"))
		}
	}
}



fn extract_focused(iquery: &iq::Query)->usize{
	for ref eq in &iquery.equations{
		match &eq.left{
			&iq::Operand::Column(ref column) => {
				if column == "focused"{
					match &eq.right{
						&iq::Operand::Number(number) => {
							let focused = number as usize;
							return focused;
						},
						_ => () 
					}
				}
			},
			_ => ()
		}

	}	
	0
}

#[derive(Debug)]
pub struct ParseError{
	desc: String,
}

impl ParseError{
	
	pub fn new(m: &str)->Self{
		ParseError{desc: m.to_owned()}
	}
}

