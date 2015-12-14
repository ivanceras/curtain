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
use global::CachePool;
use rustorm::table::Table;
use global::DatabasePool;
use unicase::UniCase;

fn get_tables(req: &mut Request, db_dev:&DatabaseDev)->Vec<Table>{
    let tables = CachePool::get_cached_tables(req);
    if !tables.is_empty(){
        info!("GOT tables from CACHE ---->>>");
        return tables;
    }else{
        let db_tables = generator::get_all_tables(db_dev);
        CachePool::cache_tables(req, db_tables.clone());
        let cached_tables = CachePool::get_cached_tables(req);
        if !cached_tables.is_empty(){
            return cached_tables;
        }else{
            error!("Cached seems not working..");
            return db_tables;
        }
        
    }
}
/// get a matching table 
pub fn get_matching_table(req: &mut Request, db: &DatabaseDev, arg_table_name: &str)->Option<Table>{
    let tables = get_tables(req, db);
    let arg_table = TableName::from_str(arg_table_name);
    for table in tables{
        if arg_table.schema.is_some(){
            let schema = arg_table.schema.as_ref().unwrap();
            if table.schema == *schema && table.name == arg_table.name{
                return Some(table.clone())
            }
        }else{
            if table.name == arg_table.name{
                return Some(table.clone())
            }
        }
    }
    None
}

fn get_windows(req: &mut Request, db_dev:&DatabaseDev)->Vec<Window>{
    let tables = get_tables(req, db_dev);
    let windows = CachePool::get_cached_windows(req);
    if !windows.is_empty(){
        info!("GOT windows from CACHE ---->>>");
        return windows;
    }else{
        let db_windows = window::extract_windows(&tables);
        CachePool::cache_windows(req, db_windows.clone());
        let cached_windows = CachePool::get_cached_windows(req);
        if !cached_windows.is_empty(){
            return cached_windows;
        }else{
            error!("Cached seems not working..");
            return db_windows;
        }
        
    }
}

/// retrive the window definition of a table
pub fn retrieve_window_api(req: &mut Request, db_dev:&DatabaseDev, arg_table_name: &str)->Result<Window, String>{
    info!("getting window: {}", arg_table_name);
    let windows = get_windows(req, db_dev);
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
pub fn list_window_api(req: &mut Request, db_dev:&DatabaseDev)->Result<Vec<Window>, String>{
    let tables = get_tables(req, db_dev);
    let windows = window::list_windows_summary(&tables);
    Ok(windows)
}

pub fn get_window(req: &mut Request) -> IronResult<Response> {
    let db = DatabasePool::get_connection(req);
    let table_name = match req.extensions.get::<Router>().unwrap().find("table"){
        Some(table_name) => Some(table_name.to_string()),
        None => None
    };
    match table_name{
        Some(ref table_name) => {
            match db {
                Ok(db) => {
                    match retrieve_window_api(req, db.as_dev(), table_name){
                        Ok(window) => {
                            let encoded = json::encode(&window).unwrap();
                            return Ok(Response::with((Status::Ok, encoded)));
                        },
                        Err(e) => {
                            return Ok(Response::with((Status::BadRequest, format!("{}",e))));
                        }
                    }
                },
                Err(e) => {
                    return Ok(Response::with((status::BadRequest, "Unable to connect to database")));
                }
            }
            
            
        },
        None =>{
             return Ok(Response::with((Status::BadRequest, "No table specified")))
        }
    }
}


pub fn list_window(req: &mut Request) -> IronResult<Response> {
    let db = DatabasePool::get_connection(req);
    match db {
        Ok(db) => {
            match list_window_api(req, db.as_dev()){
                Ok(window_list) => {
                    let encoded = json::encode(&window_list).unwrap();
                    return Ok(Response::with((status::Ok, encoded)));
                },
                Err(e) => {
                    return Ok(Response::with((status::BadRequest, format!("{}",e))));
                }
            }
        },
        Err(e) => {
            return Ok(Response::with((status::BadRequest, "Can not create database connection")));
        }
    }
}
