use iron::status;
use router::Router;
use iron::prelude::*;
use iron::headers::*;
use iron::status::Status;
use rustc_serialize::json::{self};
use std::sync::{Arc,RwLock};

use rustorm::database::DatabaseDev;
use rustorm::query::TableName;
use window_service::window::{self, Window};
use rustorm::table::Table;
use global::GlobalPools;
use global;
use std::collections::HashSet;
use global::Context;


/// try retrieving tables from cache, if none, then from db and cache it
fn get_tables(context: &mut Context)->Vec<Table>{
	let has_cache = context.has_cached_tables();
	if has_cache{
		context.get_cached_tables().unwrap()
	}else{
		get_tables_from_db_then_cache_in_context(context)
	}
}

pub fn get_tables_from_db_then_cache_in_context(context: &mut Context)->Vec<Table>{
        let db_tables = get_all_tables(context.db_dev().unwrap());
        context.cache_tables(db_tables.clone());
        db_tables
}

fn get_windows(context: &mut Context)->Vec<Window>{
    let tables = get_tables(context);
	let has_cache = context.has_cached_windows();
	if has_cache{
		context.get_cached_windows().unwrap()
	}else{
		get_windows_from_db_then_cache(context, &tables)
	}
}

fn get_windows_from_db_then_cache(context: &mut Context, tables: &Vec<Table>)->Vec<Window>{
	let db_windows = window::extract_windows(tables);
	context.cache_windows(db_windows.clone());
	db_windows
}
/// get a matching table 
pub fn get_matching_table(context: &mut Context, arg_table_name: &str)->Option<Table>{
    let tables = get_tables(context);
    let arg_table = TableName::from_str(arg_table_name);
    //check for exact match first
	for table in &tables{
        if arg_table.schema.is_some(){
            if table.schema == arg_table.schema && table.name == arg_table.name{
                return Some(table.clone())
            }
        }
    }
	//then check for table names only regardless of schema
    for table in &tables{
		if table.name == arg_table.name{
			return Some(table.to_owned())
		}
    }
    None
}

//check if the table exist
pub fn table_exist(context :&mut Context, arg_table_name: &str)->bool{
    get_matching_table(context, arg_table_name).is_some()
}

//check if this is a column in the database
pub fn all_column_names(context: &mut Context)->HashSet<String>{
   let tables = get_tables(context);
   let mut columns:HashSet<String> = HashSet::new();
   for tbl in tables{
        for col in tbl.columns{
            columns.insert(col.name.to_owned());
        }
   }
   columns
}

pub fn all_table_names(context: &mut Context)->HashSet<String>{
    let all_tables = get_tables(context);
    let mut table_names = HashSet::new();
    for table in all_tables{
        table_names.insert(table.name.to_owned()); 
    }
    table_names
}

pub fn all_schema_names(context: &mut Context)->HashSet<String>{
	let all_tables = get_tables(context);
	let mut schema_names = HashSet::new();
	for table in all_tables{
		if let Some(schema) = table.schema{
			schema_names.insert(schema.to_owned());
		}
	}
	schema_names
}


/// retrive the window definition of a table
pub fn retrieve_window(context :&mut Context, arg_table_name: &str)->Result<Window, String>{
    info!("getting window: {}", arg_table_name);
    let windows = get_windows(context);
    let table_name  = TableName::from_str(arg_table_name);
    let schema = table_name.schema;
    if schema.is_some(){
        for win in windows{
            if win.table == table_name.name && win.schema == schema{
                return Ok(win.clone());
            }
        }
    }
    else{
        for win in windows{
            if win.table == table_name.name{
                return Ok(win.clone());
            }
        }
    }
    Err(format!("No window for {}",arg_table_name))
}


/// list down the windows using only the summaries
pub fn list_window(context: &mut Context)->Result<Vec<Window>, String>{
    let tables = get_tables(context);
    let windows = window::list_windows_summary(&tables);
    Ok(windows)
}

pub fn get_window(context: &mut Context, table: &str) -> IronResult<Response> {
	match retrieve_window(context, table){
		Ok(window) => {
			let encoded = json::encode(&window).unwrap();
			return Ok(Response::with((Status::Ok, encoded)));
		},
		Err(e) => {
			return Ok(Response::with((Status::BadRequest, format!("{}",e))));
		}
	}
}


///
/// retrieve all the table definition in the database
///
pub fn get_all_tables(db_dev: &DatabaseDev) -> Vec<Table> {
    let all_tables_names = db_dev.get_all_tables();
    let mut all_table_def: Vec<Table> = Vec::new();
    for (schema, table, is_view) in all_tables_names {
        println!("Extracted {}.{}", schema, table);
        let meta = db_dev.get_table_metadata(&schema, &table, is_view);
        all_table_def.push(meta);
    }
    all_table_def
}

