use iron::status;
use router::Router;
use iron::prelude::*;
use iron::headers::*;
use iron::status::Status;
use persistent::{Read};
use rustc_serialize::json::{self};
use codegenta::generator;

use rustorm::database::DatabaseDev;
use rustorm::query::TableName;
use window::{self, Window};
//use global::AppDb;
use global::CachePool;
use rustorm::table::Table;
use global::DatabasePool;
use rustorm::pool::ManagedPool;
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

/// need to cache tables here, instead of extracting it all over a over again
/// needs to also cache windows
fn retrieve_window_api(req: &mut Request, db_dev:&DatabaseDev, arg_table_name: &str)->Result<Window, String>{
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
    //let pool = req.get::<Read<AppDb>>().unwrap();
    let table_name = match req.extensions.get::<Router>().unwrap().find("table"){
        Some(table_name) => Some(table_name.to_string()),
        None => None
    };
    match table_name{
        Some(ref table_name) => {
            //let db = pool.connect();
            match db {
                Ok(db) => {
                    match retrieve_window_api(req, db.as_dev(), table_name){
                        Ok(window) => {
                            let encoded = json::encode(&window);
                            return create_response(status::Ok, &encoded.unwrap());
                        },
                        Err(e) => {
                            return create_response(status::BadRequest, &format!("{}",e));
                        }
                    }
                },
                Err(e) => {
                    return create_response(status::BadRequest, "Unable to connect to database");
                }
            }
            
            
        },
        None =>{
             return create_response(status::BadRequest, "No table specified")
        }
    }
}

/// TODO: make this a generic library function for reducing boilerplates
pub fn create_response(status: Status, content: &str)->IronResult<Response>{
    let mut response = Response::with((status, content));
    response.headers.set(AccessControlAllowOrigin::Any);
    response.headers.set(AccessControlAllowHeaders(vec![
        UniCase("db_url".to_owned()),
        UniCase("*".to_owned()),
    ]));
    return Ok(response)
}

pub fn preflight(req :&mut Request)->IronResult<Response>{
    let mut response = Response::with((status::Ok, "Ok"));
    response.headers.set(AccessControlAllowOrigin::Any);
    response.headers.set(AccessControlAllowHeaders(vec![
        UniCase("db_url".to_owned()),
        UniCase("*".to_owned()),
    ]));
    return Ok(response)
}
pub fn list_window(req: &mut Request) -> IronResult<Response> {
    let db = DatabasePool::get_connection(req);
    //let pool = req.get::<Read<AppDb>>().unwrap();
    //let db = pool.connect();
    match db {
        Ok(db) => {
            match list_window_api(req, db.as_dev()){
                Ok(window_list) => {
                    let encoded = json::encode(&window_list);
                    return create_response(status::Ok, &encoded.unwrap());
                },
                Err(e) => {
                    return create_response(status::BadRequest, &format!("{}",e));
                }
            }
        },
        Err(e) => {
            return create_response(status::BadRequest, "Can not create database connection");
        }
    }
}