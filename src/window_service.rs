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
	let cache = globals.get_cache(db_url);
	match cache{
		Some(cache) => {
			cache.tables
		},
		None => {
			let db_tables = generator::get_all_tables(db_dev);
			globals.cache_tables(db_url, db_tables);	
			db_tables
		} 
	}
}

/// get a matching table 
pub fn get_matching_table(globals: &mut GlobalPools, db_url: &str, db_dev: &DatabaseDev, arg_table_name: &str)->Option<Table>{
    let tables = get_tables(globals, db_url, db_dev);
    let arg_table = TableName::from_str(arg_table_name);
    //check for exact match first
	for table in tables{
        if arg_table.schema.is_some(){
            let schema = arg_table.schema.as_ref().unwrap();
            if table.schema == *schema && table.name == arg_table.name{
                return Some(table.clone())
            }
        }
    }
	//then check for table names only regardless of schema
    for table in tables{
		if table.name == arg_table.name{
			return Some(table.clone())
		}
    }
    None
}

fn get_windows(globals: &mut GlobalPools, db_url: &str, db_dev:&DatabaseDev)->Vec<Window>{
    let tables = get_tables(globals, db_url, db_dev);
	let cache = globals.get_cache(db_url);

    match cache{
		Some(cache) => cache.windows,
		None => {
			let db_windows = window::extract_windows(&tables);
			globals.cache_windows(db_url, db_windows);
			db_windows
		}
	}
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
pub fn get_window_http(req: &mut Request) -> IronResult<Response> {
    let table = match req.extensions.get::<Router>().unwrap().find("table"){
        Some(table) => table.to_owned(),
        None => panic!("No table name specified!"),
    };
	let db_url = global::get_db_url(req).unwrap();
	let mut globals = GlobalPools::from_request(req).unwrap();
	get_window(&mut globals, &db_url, &table)
	
}
pub fn get_window(globals: &mut GlobalPools, db_url: &str, table: &str) -> IronResult<Response> {
	let pool = globals.get_pool(db_url).unwrap();
	match retrieve_window_api(globals, db_url, pool.connect().unwrap().as_dev(), table){
		Ok(window) => {
			let encoded = json::encode(&window).unwrap();
			return Ok(Response::with((Status::Ok, encoded)));
		},
		Err(e) => {
			return Ok(Response::with((Status::BadRequest, format!("{}",e))));
		}
	}
}


pub fn list_window_http(req: &mut Request) -> IronResult<Response> {
	panic!("not yet");
}
pub fn list_window(globals: &mut GlobalPools, db_url:&str) -> IronResult<Response> {
	let pool = globals.get_pool(db_url).unwrap();
	match list_window_api(globals, db_url, pool.connect().unwrap().as_dev()){
		Ok(window_list) => {
			let encoded = json::encode(&window_list).unwrap();
			return Ok(Response::with((status::Ok, encoded)));
		},
		Err(e) => {
			return Ok(Response::with((status::BadRequest, format!("{}",e))));
		}
	}
}
