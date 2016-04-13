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
use std::collections::BTreeMap;
use rustorm::dao::Value;
use data_service::data_api;
use uuid::Uuid;
use rustc_serialize::json::Json;
use rustorm::database::DbError;
use error::ParseError;
use error::ServiceError;
use error::ParamError;
use window_service::window::Window;
use rustc_serialize::json::DecoderError;



pub fn complex_query(context: &mut Context, main_table: &str, url_query: &Option<String>)->Result<RestData, ServiceError>{
	let validator = DbElementValidator::from_context(context);
    let (main_table_filter, rest_table_filter) = parse_complex_url_query(main_table, url_query);
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
			let rest_data:Result<RestData, ServiceError> = retrieve_main_data(context, &main_validated, &rest_validated);
            rest_data
		},
		Err(e) => Err(ServiceError::from(e)) 
	}
}


pub fn update_data(context: &mut Context, main_table: &str, updatable_data: &str)->Result<(),ServiceError>{
	if updatable_data.trim().is_empty(){
		return Err(ServiceError::from(ParamError::new("empty updatable data")));
	}else{
		println!("updatable_data: {}", updatable_data);
		let changeset: Result<Vec<ChangeSet>, DecoderError> = json::decode(updatable_data);
		match changeset{
			Ok(changeset) => {
				let window = window_api::retrieve_window(context, main_table);
				match window{
					Ok(window) => {
						apply_data_changeset(context, &window, &changeset);
						return Ok(()); 
					},
						Err(e) => {return Err(ServiceError::from(e));}
				}
			},
				Err(e) => {
					return Err(ServiceError::from(ParseError::new(&format!("{}",e))));
				}
		}
	}
	Ok(())
}

fn apply_data_changeset(context: &mut Context, window: &Window, changesets: &Vec<ChangeSet>) -> Result<(), DbError>{
    println!("applying changeset");
	for changeset in changesets{
		apply_dao_delete(context, &changeset.deleted);
		apply_dao_update(context, &changeset.updated);
		apply_dao_insert(context, &changeset.inserted);
	}
    Ok(()) 
}

fn apply_dao_delete(context: &mut Context, deletes: &Vec<Dao>) -> Result<(), DbError>{
	for delete in deletes{
		println!("--->");
		println!("deleted : {:?}", delete);
		println!("--->");
	}
	Ok(())
}

fn apply_dao_update(context: &mut Context, updates: &Vec<DaoUpdate>) -> Result<(), DbError>{
	for update in updates{
		let min = update.minimize_update();
		println!("update original: {:?}", update.original);
		println!("--->");
		println!("min : {:?}", min);
		println!("--->");
	}	
	Ok(())
}
fn apply_dao_insert(context: &mut Context, inserts: &Vec<DaoInsert>) -> Result<(), DbError>{
	for insert in inserts{
		println!("--->");
		println!("for insert: {:?}", insert);
		println!("--->");
	}
	Ok(())
}


fn parse_complex_url_query(main_table:&str, url_query: &Option<String>)->(TableFilter, Vec<TableFilter>){
    let mut main_table_filter = TableFilter{
            table: main_table.to_owned(),
           filter: None
    };
    let mut rest_table_filter = vec![];

    if let &Some(ref query) = url_query{
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
    (main_table_filter, rest_table_filter)
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



/// retrieve the window data for all tabs involved in this window
fn retrieve_main_data(context: &mut Context, main_query: &ValidatedQuery, rest_vquery: &Vec<ValidatedQuery>)->Result<RestData, ServiceError>{
	let main_table:Table = main_query.table.clone();
	let main_window = match window_api::retrieve_window(context, &main_table.name){
		Ok(main_window) => main_window,
			Err(e) => {return Err(ServiceError::from(e));}
	};
	let mut table_dao = vec![];
	let mut mquery: Query = match main_query.query{
		Some(ref query) => query.clone(),
			None => Query::new()
	};
	println!("----->>> {:#?}", mquery);
	let main_table_name = main_table.to_table_name();
	mquery.enumerate_from_table(&main_table_name);
	mquery.from(&main_table.clone());
	let main_debug = mquery.debug_build(context.db().unwrap());
	println!("MAIN debug sql: {}", main_debug);
	let main_dao_result = {
		let db = match context.db(){
			Ok(db) => db,
			Err(e) => {return Err(ServiceError::from(e));}
		};
		match mquery.retrieve(db){
			Ok(main_dao_result) => main_dao_result,
				Err(e) => {return Err(ServiceError::from(e));}
		}
	};

    println!("main dao result: {:#?}", main_dao_result);

	// all the other table dao will not be retrieved when there is no focused record on the main tab

	if let Some(main_tab) = main_window.tab{
        let main_focused_dao = extract_focused_dao(&main_table, &main_dao_result.dao, &main_query.focus_param);
        if main_focused_dao.is_none(){// if nothing is focused, just return the table dao even without marked focused
            let main_table_dao = TableDao::from_dao_result(&main_dao_result, &main_table.complete_name());
			table_dao.push(main_table_dao);
        }
        if let Some(main_focused_dao) = extract_focused_dao(&main_table, &main_dao_result.dao, &main_query.focus_param){
			let main_dao_state = mark_focused_record(&main_dao_result.dao, &main_focused_dao);
			let main_table_dao = TableDao{
									table: main_table.complete_name(), 
									dao_list: main_dao_state
								};
			table_dao.push(main_table_dao);

			let main_focused_filter = create_main_query_join_filter_from_focused_dao(&main_table, &main_focused_dao);
			let main_filter:Vec<Filter> = extract_comprehensive_filter(&main_table, &mquery);
			let mut main_with_focused_filter = main_filter.clone();
			main_with_focused_filter.extend_from_slice(&main_focused_filter);
			if let &Some(ref ext_tabs) = &main_tab.ext_tabs{
				for ext_tab in ext_tabs{
					let ext_table = match window_api::get_matching_table(context, &ext_tab.table){
						Some(ext_table) => ext_table,
							None => {return Err(ServiceError::new("Unable to get table for extension"));}
					};
					let mut ext_query = build_query(&main_table, &ext_table, &main_with_focused_filter, rest_vquery);
					let debug = ext_query.debug_build(context.db().unwrap());
					println!("debug sql: {}", debug);
					match ext_query.retrieve(context.db().unwrap()){
						Ok(dao_result) => {
							table_dao.push(TableDao::from_dao_result(&dao_result, &ext_table.complete_name()));
						},
							Err(e) => {
								return Err(ServiceError::from(e));
							}
					}
				}
			}
			if let &Some(ref has_many_tabs) = &main_tab.has_many_tabs{
				for has_many in has_many_tabs{
					let has_table = match window_api::get_matching_table(context, &has_many.table){
						Some(has_table) => has_table,
							None => {return Err(ServiceError::new("Unable to get table for has many"));}
					};
					let mut has_query = build_query(&main_table, &has_table, &main_with_focused_filter, rest_vquery);
					let debug = has_query.debug_build(context.db().unwrap());
					println!("debug sql: {}", debug);
					match has_query.retrieve(context.db().unwrap()){
						Ok(dao_result) => {
							table_dao.push(TableDao::from_dao_result(&dao_result, &has_table.complete_name()));
						}
						Err(e) => {
							return Err(ServiceError::from(e));
						}
					}
				}
			}
			if let &Some(ref has_many_indirect_tabs) = &main_tab.has_many_indirect_tabs{
				for indirect in has_many_indirect_tabs{
					// will use inner join to linker table, and inner join to the indirect
					let indirect_table = match window_api::get_matching_table(context, &indirect.table){
						Some(indirect_table) => indirect_table,
							None => {return Err(ServiceError::new("Unable to get table for extension"));}
					};
					assert!(indirect.linker_table.is_some(), format!("indirect tab {} must have a linker table specified", indirect.table));
					let linker_table_name = &indirect.linker_table.as_ref().unwrap();
					let linker_table =match window_api::get_matching_table(context, &linker_table_name){
						Some(linker_table) => linker_table,
							None => { return Err(ServiceError::new("linker table can not be found")); }
					};
					let mut ind_query = build_query_with_linker(&main_table, &indirect_table, &main_with_focused_filter, &linker_table, rest_vquery);

					let debug = ind_query.debug_build(context.db().unwrap());
					println!("indirect query debug sql: {}", debug);
					match ind_query.retrieve(context.db().unwrap()){
						Ok(dao_result) => {
							table_dao.push(TableDao::from_dao_result(&dao_result, &indirect_table.complete_name()));
						}
						Err(e) => {
							return Err(ServiceError::from(e));
						}
					}

				}
			}
		}
		let rest_data = RestData{ table_dao: table_dao};
		Ok(rest_data)
	}else{
		Err(ServiceError::new(&format!("no main tab for table {}", main_table.complete_name())))
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

fn build_query_with_linker(main_table: &Table, indirect_table: &Table, 
        main_filter: &Vec<Filter>, linker_table: &Table, rest_vquery: &Vec<ValidatedQuery>
        )->Query{
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

fn extract_focused_dao(table:&Table, dao_list: &Vec<Dao>, focus_param: &Option<FocusParam>)->Option<Dao>{
	if let &Some(ref focus_param) = focus_param{
		match focus_param{
			&FocusParam::Index(index) => {
				if dao_list.len() > index {
					Some(dao_list[index].clone())
				}else{
					None
				}
			},
			&FocusParam::PrimaryBlob(ref blob) => {
				let pkeys = table.primary_columns();
				let blob: Vec<&str> = blob.split(",").collect();
				assert!(pkeys.len() ==  blob.len(), "focused_record should only have the same length as the number of primary keys");
				let mut needle: BTreeMap<String, Value> = BTreeMap::new();
				for i in 0..pkeys.len(){
					let pk = pkeys[i];
					let bvalue = blob[i];
					let value = Value::String(bvalue.to_owned());
					let corrected_value = data_api::correct_value_type(&value, &pk.data_type);
					needle.insert(pk.name.to_owned(), corrected_value);
				}
				println!("focused record map: {:#?}", needle);
				let focused_dao = find_from_dao_list(dao_list, &needle);
				println!("focused dao: {:#?}", focused_dao);
				focused_dao
			}
		}
	}else{
		None
	}
}

fn match_needle(dao:&Dao, needle: &BTreeMap<String, Value>)->bool{
	let mut matches = 0;
	for key in needle.keys(){
		let dao_value = dao.get(key);
		let needle_value = needle.get(key);
		if let Some(needle_value) = needle_value{
            if let Some(dao_value) = dao_value{
                if needle_value == dao_value{
                    println!("we have a match here");
                    matches += 1;
                }else{
                    println!("1 key didnt match");
                    return false;
                }
            }
		}
	}
	needle.keys().len() == matches
}

fn find_from_dao_list(dao_list:&Vec<Dao>, needle: &BTreeMap<String, Value>)->Option<Dao>{
	for dao in dao_list{
		if match_needle(dao, needle){
			return Some(dao.clone());
		}	
	}
	None
}

/// extract the filters of this query from main table, but adds details to the column names to avoid unambigous feild
/// if there is no table specified in the column name, the main table is added, else leave it as it is
fn extract_comprehensive_filter(main_table: &Table, main_query: &Query)->Vec<Filter>{
	let mut filters = vec![];
	for filter in &main_query.filters{
		let mut condition = filter.condition.clone();
		let left = match &condition.left{
			&Operand::ColumnName(ref column) => {
				let mut column = column.clone();
				if column.table.is_some(){
					//leave as is	
				}else{
					column.table = Some(main_table.name.to_owned())
				}
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


fn create_main_query_join_filter_from_focused_dao(table: &Table, focused_dao: &Dao)->Vec<Filter>{
	let mut filters = vec![];
	for pri in table.primary_columns(){
		let pvalue = focused_dao.get(&pri.name);
        if let Some(pvalue) = pvalue{
            let column = pri.complete_name();
            let filter = Filter::new(&column, Equality::EQ, &pvalue.to_owned());
            filters.push(filter)
        }
	}
	filters
}

#[derive(Debug)]
#[derive(RustcEncodable)]
struct DaoState{
	focused: bool,
	dao: Dao,
}

impl DaoState{

	fn from_dao(dao: Dao)->Self{
		DaoState{
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
	focus_param: Option<FocusParam>
}

#[derive(Debug)]
pub struct TableFilter{
	pub table: String,
	pub filter: Option<String>
}

impl TableFilter{

	fn transform(&self, context: &mut Context, validator: &DbElementValidator)->Result<ValidatedQuery, ParseError>{

		if validator.is_valid_table(&self.table){
			let table = match window_api::get_matching_table(context, &self.table){
				Some(table) => table,
				None => {return Err(ParseError::new("Unable to get matching table")); }
			};
			match self.filter{
				Some(ref filter) => {
					let parsed = inquerest::query(&filter);
					match parsed{
						Ok(parsed) => {
							let focus_param = extract_focus_param(&parsed);
							let transformed = parsed.transform(&validator);
							let vquery = ValidatedQuery{
								table: table,
								query: Some(transformed),
								focus_param: focus_param,
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
						focus_param: None
					};
					Ok(vquery)
				}
			}
		}else{
			Err(ParseError::new("table is invalid"))
		}
	}
}


/// prioritized focused_record than focused
fn extract_focus_param(iquery: &iq::Query)->Option<FocusParam>{
	let focused_record = find_in_equation(&iquery.equations, "focused_record");
	if let Some(focused_record) = focused_record {
		match focused_record{
			&iq::Operand::Number(number) => {
				let primary_blob = format!("{}",number);
				return Some(FocusParam::PrimaryBlob(primary_blob))
			},
				&iq::Operand::Column(ref column) => {
					// example: focused_record=efc62342-1230af,100001
					let primary_blob = format!("{}",column);
					return Some(FocusParam::PrimaryBlob(primary_blob))
				},
				&iq::Operand::Boolean(value)=>{
					let primary_blob = format!("{}", value);
					return Some(FocusParam::PrimaryBlob(primary_blob))
				},
				_ => ()
		}
	}
	let focused = find_in_equation(&iquery.equations, "focused");
	if let Some(focused) = focused{
		match focused{
			&iq::Operand::Number(number) => {
				let focused = number as usize;
				return Some(FocusParam::Index(focused));
			},
				_ => () 
		}
	}
	None
}

/// return the right operand of the equation when this colum match
fn find_in_equation<'a>(equations: &'a Vec<iq::Equation>, left_column: &str)->Option<&'a iq::Operand>{
	for ref eq in equations{
		match &eq.left{
			&iq::Operand::Column(ref column) => {
				if column == left_column{
					return Some(&eq.right)
				}
			},
			_ => ()
		}
	}	
	None
}

/// index will be the relative position of the record
/// record will be transformed into a filter of the primary keys
/// comma separated values of the primary key value with respect to the
/// position order of the primary keys in the table
#[derive(Debug)]
enum FocusParam{
	Index(usize),
	PrimaryBlob(String)
}


fn mark_focused_record(dao_list: &Vec<Dao>, focused_dao: &Dao)->Vec<DaoState>{
	let mut dao_states = vec![];
	for dao in dao_list{
		if dao == focused_dao{
			let dao_state = DaoState{focused: true, dao: dao.clone()};
			dao_states.push(dao_state);
		}else{
			dao_states.push(DaoState::from_dao(dao.clone()));
		}
	}
	dao_states
}


/// when a dao is updated
#[derive(Debug)]
#[derive(RustcEncodable)]
#[derive(RustcDecodable)]
pub struct DaoUpdate{
	pub original: Dao,
	pub updated: Dao,
}

impl DaoUpdate{
		
	/// get only the minimal update by including only the 
	/// changed values
	pub fn minimize_update(&self)->Dao{
		let mut changeset = Dao::new();
		let keys = self.original.keys();
		for key in keys{
			println!("key: {:?}", key);
			let updated_value = self.updated.get(key);
			let orig_value = self.original.get(key);
            if updated_value == orig_value{
                println!("no change for {}", key);
            }else{
                if let Some(updated_value) = updated_value{
                    changeset.insert(key.to_owned(), updated_value.to_owned());
                }
            }
		}
		changeset
	}
}

#[test]
fn test_dao_minimize_update(){
	let mut dao1 = Dao::new();
	dao1.insert("key1".to_owned(),Value::String("value1".to_owned()));
	dao1.insert("key2".to_owned(),Value::String("value2".to_owned()));

	let mut dao2 = dao1.clone();
	dao2.insert("key2".to_owned(),Value::String("I changed this".to_owned()));

	let update = DaoUpdate{
		original: dao1.clone(),
		updated: dao2.clone()
	};
	let min = update.minimize_update();
	println!("min: {:#?}",min);
	let mut expected = Dao::new();
	expected.insert("key2".to_owned(), Value::String("I changed this".to_owned()));

	assert_eq!(expected, min);
}

/// Dao for inserting, but with record ID to
/// identify which record it is before actually inserting into the database
/// which may change the primary key value, due to database custom function
/// that may have been used to generate it.
/// The record_id is useful when a referring record from some other
/// table is also created which also refers to this main record.
/// This is akin to simultaneously inserting relative records into different table
/// the primary key of the main record will have to return and will be used in the referring table
/// for the succedding insert operation
#[derive(Debug)]
#[derive(RustcEncodable)]
#[derive(RustcDecodable)]
pub struct DaoInsert{
	dao: Dao,
	pub referred_record_id: Option<Uuid>, // the main record_id to refer to
	pub record_id: Uuid, //all record to be inserted must have this
}

impl DaoInsert{
	
	/// use this when inserting to main table
	pub fn from_dao(dao: &Dao)->Self{
		DaoInsert{
			dao: dao.clone(),
			referred_record_id: None,
			record_id: Uuid::new_v4()
		}
	}

	/// use this when inserting in ext_table, has_many, indirect tabs
	/// and when the main record has just been entered but will batch together

	pub fn new_with_new_referred_record(dao: &Dao, referred_record_id: &Uuid)->Self{
		DaoInsert{
			dao: dao.clone(),
			referred_record_id: Some(referred_record_id.clone()),
			record_id: Uuid::new_v4(),
		}
	}
}

/// the changeset of dao in a table
/// all the inserted dao will be inserted first
/// all the updated ones will be updated 2nd
/// all the deleted ones will be deleted last
/// deletion may cause referential integrity errors
/// rename to BatchData?
#[derive(Debug)]
#[derive(RustcEncodable)]
#[derive(RustcDecodable)]
pub struct ChangeSet{
	pub table: String,
	pub inserted: Vec<DaoInsert>,
	pub deleted: Vec<Dao>,// will use the primary key value if avaialble, else the uniques, else all the matching record
	pub updated: Vec<DaoUpdate>
}


