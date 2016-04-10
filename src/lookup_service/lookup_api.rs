

//! lookup stores the dao in this struct
//! if the table being lookup is relatively small
//! lesser than 20 records will all be retrieved
//! and will be rendered as dropdown in the client side
//! or combo box with associated image next to the identifier
//! of the record
//!
//! If the table size is relatively high, which is greater than 20 records
//! It will be rendered as a lookup search

use global::Context;
use window_service::window_api;
use rustorm::dao::Dao;
use rustorm::dao::DaoResult;
use rustorm::query::Query;
use rustorm::query::table_name::ToTableName;
use rustorm::database::DbError;
use rustorm::table::Table;
use window_service::window::{Window,Tab};

#[derive(Debug)]
#[derive(RustcEncodable)]
pub struct LookupTable{
	table: String,
	dao_list: Vec<Dao>,
}

impl LookupTable{
	
	fn from_dao_result(dao_result: &DaoResult, table_name: &str)->Self{
		LookupTable{
			table: table_name.to_owned(),
			dao_list: dao_result.dao.clone()
		}
	}

}

pub fn get_lookup_data(context: &mut Context, table: &str)->Result<Vec<LookupTable>, DbError>{
	let window = window_api::retrieve_window(context, table);
	match window{
		Ok(ref window) => retrieve_lookup_tables_from_window(context, window),
		Err(e) => Err(DbError::new("unable to retrieve table"))
	}
}

pub fn retrieve_lookup_tables_from_window(context: &mut Context, window: &Window)->Result<Vec<LookupTable>, DbError>{
	let lookup_tables = get_lookup_tables(context, window);
	match lookup_tables{
		Ok(lookup_tables) => retrieve_data_from_lookup_table(context, &lookup_tables),
		Err(e) => Err(e)
	}
}

/// all fields that is a table reference
fn get_lookup_tables(context: &mut Context, window: &Window)->Result<Vec<Table>,DbError>{
	println!("Getting lookup tables for {}", window.name);	
	let mut tables:Vec<Table> = vec![];
	if let Some(ref tab) = window.tab{
		let tab_lookups = get_lookup_tables_from_tab(context, tab);
		for tl in tab_lookups{
			if tables.contains(&tl){
				warn!("this table is already added");
			}else{
				tables.push(tl);
			}
		}
	}
	Ok(tables)
}

trait Contains{
	fn contains(&self, table: &Table)->bool;	
}

impl Contains for Vec<Table>{
	
	fn contains(&self, table: &Table)->bool{
		for tb in self{
			if tb == table{
				return true;
			}
		}
		false
	}
}

fn get_lookup_tables_from_tab(context: &mut Context, tab: &Tab)->Vec<Table>{
	let mut tables: Vec<Table> = vec![];
	for ref field in &tab.fields{
		if field.reference == "Table"{
			assert!(field.reference_value.is_some(), "Table lookup should be specified");
			if let Some(ref ref_table_name) = field.reference_value{
				let table = window_api::get_matching_table(context, ref_table_name);
				if let Some(table) = table{
					tables.push(table);
				}else{
					warn!("Unable to get matching table");
				}
			}
		}
	}
	if let Some(ref ext_tabs) = tab.ext_tabs{
		for ext_tab in ext_tabs{
			let ext_ltables = get_lookup_tables_from_tab(context, &ext_tab);
			for ext_ltable in ext_ltables{
				tables.push(ext_ltable);
			}
		}
	}
	if let Some(ref has_many_tabs) = tab.has_many_tabs{
		for has_tab in has_many_tabs{
			let has_ltables = get_lookup_tables_from_tab(context, &has_tab);
			for has_ltable in has_ltables{
				tables.push(has_ltable);
			}
		}
	}
	if let Some(ref has_many_indirect_tabs) = tab.has_many_indirect_tabs{
		for ind_tab in has_many_indirect_tabs{
			let ind_ltables = get_lookup_tables_from_tab(context, &ind_tab);
			for ind_ltable in ind_ltables{
				tables.push(ind_ltable);
			}
		}
	}
	tables
}

fn retrieve_data_from_lookup_table(context: &mut Context, tables: &Vec<Table>)->Result<Vec<LookupTable>,DbError>{
	let mut lookup_tables: Vec<LookupTable> = vec![];
	for table in tables{
		let est_row_count =  match context.db_dev(){
			Ok(db_dev) => {
				let ref_schema = match table.schema{
					Some(ref schema) => schema.to_owned(),
						None => panic!("there should be schema")
				};
				println!("schema: {}, table: {}",ref_schema, table.name);
				let est_row_count = db_dev.get_row_count_estimate(&ref_schema, &table.name);	
				est_row_count
			}
			Err(e) => {return Err(e);}
		};
		if let Some(est_row_count) = est_row_count{
			let thresh_hold = 20;
			if est_row_count < thresh_hold{
				//retrieve_lookup_dao
				let table_name = table.to_table_name();
				let mut query = Query::select();
				query.enumerate_from_table(&table_name);
				query.from(table);
				query.set_limit(thresh_hold);
				let debug = query.debug_build(context.db().unwrap());
				println!("debug sql: {}", debug);
				let dao_result = match query.retrieve(context.db().unwrap()){
					Ok(dao_result) => dao_result,
						Err(e) =>  {return Err(e);}
				};
				let ltable = LookupTable::from_dao_result(&dao_result, &table.complete_name());
				lookup_tables.push(ltable);
			}
		}
	}
	Ok(lookup_tables)
}
