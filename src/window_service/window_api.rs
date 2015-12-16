use iron::status;
use router::Router;
use iron::prelude::*;
use iron::headers::*;
use iron::status::Status;
use rustc_serialize::json::{self};
use codegenta::generator;
use std::sync::{Arc,RwLock};

use rustorm::database::DatabaseDev;
use rustorm::query::TableName;
use window_service::window::{self, Window};
use rustorm::table::Table;
use unicase::UniCase;
use global::GlobalPools;
use global;


/// try retrieving tables from cache, if none, then from db and cache it
fn get_tables(globals: Arc<RwLock<GlobalPools>>, db_url: &str, db_dev:&DatabaseDev)->Vec<Table>{
	let has_cache = globals.read().unwrap().has_cached_tables(db_url);
	if has_cache{
		globals.read().unwrap().get_cached_tables(db_url).unwrap()
	}else{
		get_tables_from_db_then_cache(globals, db_url, db_dev)
	}
}

pub fn get_tables_from_db_then_cache(globals: Arc<RwLock<GlobalPools>>, db_url: &str, db_dev:&DatabaseDev)->Vec<Table>{
	let db_tables = generator::get_all_tables(db_dev);
	globals.write().unwrap().cache_tables(db_url, db_tables.clone());	
	db_tables
}

fn get_windows(globals: Arc<RwLock<GlobalPools>>, db_url: &str, db_dev:&DatabaseDev)->Vec<Window>{
    let tables = get_tables(globals.clone(), db_url, db_dev);
	let has_cache = globals.read().unwrap().has_cached_windows(db_url);
	if has_cache{
		globals.read().unwrap().get_cached_windows(db_url).unwrap()
	}else{
		get_windows_from_db_then_cache(globals, db_url, &tables, db_dev)
	}
}

fn get_windows_from_db_then_cache(globals: Arc<RwLock<GlobalPools>>, db_url: &str, tables: &Vec<Table>, db_dev: &DatabaseDev)->Vec<Window>{
	let db_windows = window::extract_windows(tables);
	globals.write().unwrap().cache_windows(db_url, db_windows.clone());
	db_windows
}
/// get a matching table 
pub fn get_matching_table(globals: Arc<RwLock<GlobalPools>>, db_url: &str, db_dev: &DatabaseDev, arg_table_name: &str)->Option<Table>{
    let tables = get_tables(globals, db_url, db_dev);
    let arg_table = TableName::from_str(arg_table_name);
    //check for exact match first
	for table in &tables{
        if arg_table.schema.is_some(){
            let schema = arg_table.schema.as_ref().unwrap();
            if table.schema == *schema && table.name == arg_table.name{
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


/// retrive the window definition of a table
pub fn retrieve_window(globals: Arc<RwLock<GlobalPools>>, db_url: &str, db_dev:&DatabaseDev, arg_table_name: &str)->Result<Window, String>{
    info!("getting window: {}", arg_table_name);
    let windows = get_windows(globals, db_url, db_dev);
    let table_name  = TableName::from_str(arg_table_name);
    let schema = table_name.schema;
    if schema.is_some(){
        let schema = schema.unwrap();
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
pub fn list_window(globals: Arc<RwLock<GlobalPools>>, db_url: &str, db_dev:&DatabaseDev)->Result<Vec<Window>, String>{
    let tables = get_tables(globals, db_url, db_dev);
    let windows = window::list_windows_summary(&tables);
    Ok(windows)
}

pub fn get_window(globals: Arc<RwLock<GlobalPools>>, db_url: &str, table: &str) -> IronResult<Response> {
	let platform = globals.write().unwrap().get_connection(db_url).unwrap();
	match retrieve_window(globals, db_url, platform.as_dev(), table){
		Ok(window) => {
			let encoded = json::encode(&window).unwrap();
			return Ok(Response::with((Status::Ok, encoded)));
		},
		Err(e) => {
			return Ok(Response::with((Status::BadRequest, format!("{}",e))));
		}
	}
}

