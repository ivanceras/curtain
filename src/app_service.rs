
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
use rustorm::query::Filter;

pub fn complex_query(req: &mut Request)->IronResult<Response>{
    let mut context = Context::new(req);
	let validator = DbElementValidator::from_context(&mut context);
	let complex_query = extract_complex_query(req);
	println!("complex query: {:#?}", complex_query);
	match complex_query{
		Ok((main_table_filter, rest_table_filter)) => {
			println!("main_table_filter: {:#?}", main_table_filter);
			let main_validated = main_table_filter.transform(&mut context, &validator);
			match main_validated{
				Ok(main_validated) => {
					let mut rest_validated = vec![];
					for rest in rest_table_filter{
						match rest.transform(&mut context, &validator){
							Ok(rtransformed) => {
								rest_validated.push(rtransformed);
							},
							Err(e) => ()
						}
					}
					let result = retrieve_main_data(&mut context, &main_validated, &rest_validated);
					let main_table = &main_validated.table;
					let main_window = window_api::retrieve_window(&mut context, &main_table.name);
					println!("main window: {:#?}", main_window);
					let main_filter = match result{
						Ok((dao_result, main_filter)) => main_filter,
						Err(e) => vec![]
					};
					retrieve_data_from_table(&mut context, &rest_validated[0], &main_table, &main_filter);
				},
				Err(e) => ()
			}
		}
		Err(e) => ()
	};
    let mut response = Response::with((status::Ok, "A complex query..."));
    Ok(response)
}

/// retrieve the data on this table using the query
/// based on the focused dao, a filter will be added
/// for the rest of the table that has link to the main table
/// base on the filter of the main table and the primary key,
/// all other corresponding table will be left join to tho the main table
/// and included the filter of each other corresponding table.
/// main_query -> main_dao
/// main_query + table1_query -> table_dao

struct MainQuery{
	query: Query,
	focusded_dao: Dao,
	dao_result: DaoResult,
	join_filter: Filter,
}

fn retrieve_main_data(context: &mut Context, vquery: &ValidatedQuery, rest_validated: &Vec<ValidatedQuery>)->Result<(DaoResult, Vec<Filter>), ParseError>{
	let table = vquery.table.to_owned();
	match vquery.query{
		Some(ref query) => {
			let mut query = query.clone();
			query.from(&table);
			println!("query: {:#?}", query);
			match context.db(){
				Ok(db) => {
					let dao_result = query.retrieve(db);
					match dao_result{
						Ok(dao_result) => {
							let focused_dao = &dao_result.dao[vquery.focused];
							let main_focused_filter = create_main_query_join_filter_from_focused_dao(&table, focused_dao);
							let mut main_filter:Vec<Filter> = query.filters.clone();
							main_filter.extend_from_slice(&main_focused_filter);
							println!("focused dao: {:#?}", focused_dao);
							println!("main focused filter: {:#?}", main_focused_filter);
							println!("all main_filter: {:#?}", main_filter);
							Ok((dao_result.clone(), main_filter))
						},
						Err(e) => Err(ParseError::new("unable to retrieve dao results...")) 
					}
				},
				Err(e) => Err(ParseError::new("unable to get db connection")) 
			}
		},
		None => Err(ParseError::new("query is none..."))
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
		let filter = Filter::new(&pri.name, Equality::EQ, &pvalue);
		filters.push(filter)
	}
	filters
}

/// determines whether to update, insert the data
enum DataState{
	Orig, //original record, untouched
	Inserted,// a newly inserted record
	Dirty, // the record is edited and will be updated to the database
	Deleted, // record is deleted
}
struct DaoState{
	state: DataState,
	focused: bool,
	dao: Dao,
}

struct TableDao{
	table: String,
	dao_list: Vec<DaoState>,
} 
impl TableDao{
	
	fn from(dao_result: &DaoResult)->Self{
		unimplemented!();
	}
}

#[derive(Debug)]
struct TableFilter{
	table: String,
	filter: Option<String>
}

#[derive(Debug)]
struct ComplexQuery{
	main: TableFilter,
	rest: Vec<TableFilter>,
}

#[derive(Debug)]
struct ValidatedQuery{
	table: Table,
	query: Option<Query>,
	focused: usize,
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
struct ParseError{
	desc: String,
}

impl ParseError{
	
	fn new(m: &str)->Self{
		ParseError{desc: m.to_owned()}
	}
}

fn extract_complex_query(req: &mut Request)->Result<(TableFilter, Vec<TableFilter>), ParseError>{
	println!("a complex app query");	
	let main_table = req.extensions.get::<Router>().unwrap().find("main_table");
	match main_table{
		Some(main_table) => {
			let query = &req.url.query;
			let mut main_table_filter = TableFilter{
					table: main_table.to_owned(),
					filter: None
				};
				
			let mut rest_table_filter = vec![];

			if let &Some(ref query) = &req.url.query{
				let table_queries:Vec<&str> = query.split("/").collect();
				if table_queries.len() > 0{
					main_table_filter.filter = Some(table_queries[0].to_owned());
				};
				let rest_query:Vec<&&str> = table_queries.iter().skip(1).collect();
				for q in rest_query{
					let table_filter: Vec<&str> = q.split("?").collect();
					let table = if table_filter.len() > 0 {
						Some(table_filter[0])
					}else{None};

					let filter = if table_filter.len() > 1 {
						Some(table_filter[1].to_owned())
					}else{None};

					if let Some(tbl) = table{
						let table_filter = TableFilter{
							table: tbl.to_owned(),
							filter: filter,
						};
						rest_table_filter.push(table_filter);
					}

				}
			}
			Ok((main_table_filter, rest_table_filter))
		},
		None => {
			Err(ParseError::new("No main table specified"))
		}
	}
}



#[test]
fn test_complex_query(){
		let query = "age=lt.13&(student=eq.true|gender=eq.M)&order_by=age.desc,height.asc&page=20&page_size=100&focused=0/category?name=starts_with(lee)&focused=0";
		let table_queries:Vec<&str> = query.split("/").collect();
		let main_filter:Vec<&&str> = table_queries.iter().take(1).collect();
		let main_filter = table_queries[0];
		println!("main_filter: {:?}", main_filter);
		let filter = inquerest::query(main_filter);
		println!("filte: {:#?}",filter);

		let rest_query:Vec<&&str> = table_queries.iter().skip(1).collect();
		println!("rest of query: {:?}",rest_query);
		for i in rest_query{
			let table_filter:Vec<&str> = i.split("?").collect();
			let table = table_filter[0];
			let filter = table_filter[1];
			println!("table: {}", table);
			println!("filter: {}", filter);
			let parsed_filter = inquerest::query(filter);
			println!("parsed filter: {:#?}",parsed_filter);
			assert_eq!("name=starts_with(lee)&focused=0", filter);
		}	
		assert_eq!("age=lt.13&(student=eq.true|gender=eq.M)&order_by=age.desc,height.asc&page=20&page_size=100&focused=0",main_filter);

}

