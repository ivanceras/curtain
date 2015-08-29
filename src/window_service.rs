use iron::status;
use router::Router;
use iron::prelude::*;
use iron::headers::*;
use persistent::{Read};
use rustc_serialize::json::{self};
use codegenta::generator;

use rustorm::database::DatabaseDev;
use rustorm::query::TableName;
use window::{self, Window};
use global::AppDb;

fn retrieve_window_api(db_dev:&DatabaseDev, arg_table_name: &str)->Result<Window, String>{
    println!("getting window: {}", arg_table_name);
    let tables = generator::get_all_tables(db_dev);
    let windows = window::extract_windows(&tables);
    let table_name  = TableName::from_str(arg_table_name);
    let schema = table_name.schema;
    if schema.is_some(){
        let schema = schema.unwrap();
        for win in windows{
            if win.table == table_name.name && win.schema == schema{
                return Ok(win);
            }
        }
    }
    else{
        for win in windows{
            if win.table == table_name.name{
                return Ok(win);
            }
        }
    }
    Err(format!("No window for {}",arg_table_name))
}

pub fn list_window_api(db_dev:&DatabaseDev)->Result<Vec<Window>, String>{
    let tables = generator::get_all_tables(db_dev);
    let windows = window::list_windows_summary(&tables);
    Ok(windows)
}

pub fn get_window(req: &mut Request) -> IronResult<Response> {
    let pool = req.get::<Read<AppDb>>().unwrap();
    let table_name = req.extensions.get::<Router>().unwrap().find("table");
    match table_name{
        Some(ref table_name) => {
            println!("table_name: {:?}", table_name);
            let db = pool.connect();
            match db {
                Ok(db) => {
                    match retrieve_window_api(db.as_dev(), table_name){
                        Ok(window) => {
                            let encoded = json::encode(&window);
                            let mut response = Response::with((status::Ok, encoded.unwrap()));
                            response.headers.set(AccessControlAllowOrigin::Any);
                            return Ok(response)
                        },
                        Err(e) => {
                            let mut response = Response::with((status::BadRequest, e));
                            response.headers.set(AccessControlAllowOrigin::Any);
                            return Ok(response)
                        }
                    }
                },
                Err(e) => return Ok(Response::with((status::BadRequest, "Unable to connect to database")))
            }
            
            
        },
        None =>{
             return Ok(Response::with((status::BadRequest, "No table specified")))
        }
    }
}

pub fn list_window(req: &mut Request) -> IronResult<Response> {
    let pool = req.get::<Read<AppDb>>().unwrap();
    let db = pool.connect();
    match db {
        Ok(db) => {
            match list_window_api(db.as_dev()){
                Ok(window_list) => {
                    let encoded = json::encode(&window_list);
                    let mut response = Response::with((status::Ok, encoded.unwrap()));
                    response.headers.set(AccessControlAllowOrigin::Any);
                    return Ok(response)
                },
                Err(e) => {
                    let mut response = Response::with((status::BadRequest, e));
                    response.headers.set(AccessControlAllowOrigin::Any);
                    return Ok(response)
                }
            }
        },
        Err(e) => return Ok(Response::with((status::BadRequest, "Unable to connect to database")))
    }
}