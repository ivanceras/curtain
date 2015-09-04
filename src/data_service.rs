use iron::status;
use router::Router;
use iron::prelude::*;
use iron::headers::*;
use rustc_serialize::json::{self};

use rustorm::database::Database;
use rustorm::dao::{SerDaoResult};
use rustorm::database::DbError;
use rustorm::query::Query;
//use global::AppDb;
use global::SessionHash;
use std::io::Read;
use global::DatabasePool;
use window_service;

pub fn retrieve_data(db: &Database, table: &str, page_size: usize)->Result<SerDaoResult, DbError>{
    let mut query = Query::select_all();
    let result = query.from(&table)
          .set_page_size(page_size)
          .retrieve(db);
    match result{
        Ok(result) => Ok(SerDaoResult::from_dao_result(result)),
        Err(e) => Err(e)
    }
}

pub fn get_data(req: &mut Request) -> IronResult<Response> {
//    let pool = req.get::<PRead<AppDb>>().unwrap();
    let db = DatabasePool::get_connection(req);
    let table_name = req.extensions.get::<Router>().unwrap().find("table");
    let page_size = 20;
    println!("query: {:?}", req.url.query);
    match table_name{
        Some(ref table_name) => {
            println!("table_name: {:?}", table_name);
//            let db = pool.connect();
            match db {
                Ok(db) => {
                    let data = retrieve_data(db.as_ref(), table_name, page_size);
                    match data{
                        Ok(data) => {
                                let encoded = json::encode(&data);
                                return window_service::create_response(status::Ok, &encoded.unwrap());
                            },
                        Err(e) => {
                                return window_service::create_response(status::BadRequest, &format!("{}",e));
                            }
                    }
                },
                Err(e) => return window_service::create_response(status::BadRequest, "Unable to connect to database")
            }
            
            
        },
        None =>{
             return window_service::create_response(status::BadRequest, "No table specified")
        }
    }
}

pub fn set_db_url(req: &mut Request) -> IronResult<Response> {
    println!("Setting db url...");
    let mut content = String::new();
    req.body.read_to_string(&mut content).unwrap();
    println!("content: {}",content);
    let db_url = content;
    SessionHash::set_db_url(req, &db_url);
    return window_service::create_response(status::Ok, "Ok");
}