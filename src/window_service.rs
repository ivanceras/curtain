use iron::status;
use router::Router;
use iron::prelude::*;
use iron::headers::*;
use iron::status::Status;
use rustc_serialize::json::{self};
use codegenta::generator;

use rustorm::database::DatabaseDev;
use rustorm::query::TableName;
use window::{self, Window};
use rustorm::table::Table;
use unicase::UniCase;
use global::GlobalPools;
use global;

/// try retrieving tables from cache, if none, then from db and cache it
fn get_tables(globals: &mut GlobalPools, db_url: &str, db_dev:&DatabaseDev)->Vec<Table>{
	let has_cache = globals.has_cache(db_url);
	if has_cache{
		//let cache = globals.get_cache(db_url).unwrap();
		//if cache.tables.is_some(){
		//	return cache.tables.unwrap().clone()
		//}
		get_tables_from_db_then_cache(globals, db_url, db_dev)
	}else{
		get_tables_from_db_then_cache(globals, db_url, db_dev)
	}
}

pub fn get_tables_from_db_then_cache(globals: &mut GlobalPools, db_url: &str, db_dev:&DatabaseDev)->Vec<Table>{
	let db_tables = generator::get_all_tables(db_dev);
	globals.cache_tables(db_url, db_tables.clone());	
	db_tables
}

fn get_windows(globals: &mut GlobalPools, db_url: &str, db_dev:&DatabaseDev)->Vec<Window>{
    let tables = get_tables(globals, db_url, db_dev);
	let has_cache = globals.has_cache(db_url);
	if has_cache{
		//let cache = globals.get_cache(db_url).unwrap();
		//match &cache.windows{
		//	&Some(ref windows) => windows.clone(),
		//	&None => get_windows_from_db_then_cache(globals, db_url, &tables, db_dev)
		//}
		get_windows_from_db_then_cache(globals, db_url, &tables, db_dev)
	}else{
		get_windows_from_db_then_cache(globals, db_url, &tables, db_dev)
	}
}

fn get_windows_from_db_then_cache(globals: &mut GlobalPools, db_url: &str, tables: &Vec<Table>, db_dev: &DatabaseDev)->Vec<Window>{
	let db_windows = window::extract_windows(tables);
	globals.cache_windows(db_url, db_windows.clone());
	db_windows
}
/// get a matching table 
pub fn get_matching_table(globals: &mut GlobalPools, db_url: &str, db_dev: &DatabaseDev, arg_table_name: &str)->Option<Table>{
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
pub fn retrieve_window_api(globals: &mut GlobalPools, db_url: &str, db_dev:&DatabaseDev, arg_table_name: &str)->Result<Window, String>{
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
pub fn list_window_api(globals: &mut GlobalPools, db_url: &str, db_dev:&DatabaseDev)->Result<Vec<Window>, String>{
    let tables = get_tables(globals, db_url, db_dev);
    let windows = window::list_windows_summary(&tables);
    Ok(windows)
}

/// http request
pub fn http_get_window(req: &mut Request) -> IronResult<Response> {
    let table = match req.extensions.get::<Router>().unwrap().find("table"){
        Some(table) => table.to_owned(),
        None => panic!("No table name specified!"),
    };
	let db_url = global::get_db_url(req).unwrap();
	let arc = GlobalPools::from_request(req);
	let mut globals = arc.lock().unwrap();
	get_window(&mut globals, &db_url, &table)
	
}
pub fn get_window<'a>(globals: &'a mut GlobalPools, db_url: &str, table: &str) -> IronResult<Response> {
	let platform = globals.get_connection(db_url).unwrap();
	match retrieve_window_api(globals, db_url, platform.as_dev(), table){
		Ok(window) => {
			let encoded = json::encode(&window).unwrap();
			return Ok(Response::with((Status::Ok, encoded)));
		},
		Err(e) => {
			return Ok(Response::with((Status::BadRequest, format!("{}",e))));
		}
	}
}


pub fn http_list_window(req: &mut Request) -> IronResult<Response> {
	let db_url = global::get_db_url(req).unwrap();
	let arc = GlobalPools::from_request(req);
	let mut globals = arc.lock().unwrap();
	list_window(&mut globals, &db_url)
}
pub fn list_window(globals: &mut GlobalPools, db_url:&str) -> IronResult<Response> {
	let platform = globals.get_connection(db_url).unwrap();
	match list_window_api(globals, db_url, platform.as_dev()){
		Ok(window_list) => {
			let encoded = json::encode(&window_list).unwrap();
			return Ok(Response::with((status::Ok, encoded)));
		},
		Err(e) => {
			return Ok(Response::with((status::BadRequest, format!("{}",e))));
		}
	}
}
