use iron::status;
use router::Router;
use iron::prelude::*;
use rustc_serialize::json::{self};

use rustorm::database::Database;
use rustorm::dao::{SerDaoResult};
use rustorm::database::DbError;
use rustorm::query::Query;
use global::SessionHash;
use std::io::Read;
use global::DatabasePool;
use response;
use queryst;
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
    let db = DatabasePool::get_connection(req);
    let table_name = req.extensions.get::<Router>().unwrap().find("table");
    let page_size = 20;
    println!("query: {:?}", req.url.query);
    match table_name{
        Some(ref table_name) => {
            println!("table_name: {:?}", table_name);
            match db {
                Ok(db) => {
                    let data = retrieve_data(db.as_ref(), table_name, page_size);
                    match data{
                        Ok(data) => {
                                let encoded = json::encode(&data);
                                return response::create_response(status::Ok, &encoded.unwrap());
                            },
                        Err(e) => {
                                return response::create_response(status::BadRequest, &format!("{}",e));
                            }
                    }
                },
                Err(e) => return response::create_response(status::BadRequest, "Unable to connect to database")
            }
            
            
        },
        None =>{
             return response::create_response(status::BadRequest, "No table specified")
        }
    }
}

/// extracts the details of the record,
/// it will extract the data of the tables that is linked to it

pub fn table_detail(req: &mut Request) -> IronResult<Response> {
    println!("Extracting table detail..");
    let arg_table = {// to get away with mutable borrow on req, end the borrow on req.extensions right away
        let table_name = req.extensions.get::<Router>().unwrap().find("table");
        match table_name{
            Some(table_name) => Some(table_name.to_string()),
            None => None
        }
    };
    if arg_table.is_none(){
        return response::create_response(status::BadRequest, "No table specified")
    }
    let page_size = 20;//page will be sent in the range header
    println!("table: {:?}", arg_table);
    let db = DatabasePool::get_connection(req);
    match db{
        Err(e) => response::create_response(status::BadRequest, "Unable to connect to Database"),
        Ok(db) => {
            let arg_table = arg_table.unwrap();
            let window = window_service::retrieve_window_api(req, db.as_dev(), &arg_table);
            match window{
                Err(e) => response::create_response(status::BadRequest, "No table with that name"),
                Ok(window) => {
                    println!("query: {:?}", req.url.query);
                    println!("window: {} {}", window.name, window.table);
                    let table = window_service::get_matching_table(req, db.as_dev(), &window.table).unwrap();
                    let tab = window.tab.unwrap();
                    for ext_tab in tab.ext_tabs.unwrap(){
                        println!("ext_tab: {}",ext_tab.table);
                        let mut query = Query::select_all();
                                query.from_table(&ext_tab.table)
                                    .left_join_table(&table.complete_name(), 
                                        "", "");
                                
                    }
                    for has_many in tab.has_many_tabs.unwrap(){
                        println!("has_many: {}",has_many.table);
                    }
                     for indirect in tab.has_many_indirect_tabs.unwrap(){
                        println!("has_many_indirect_tabs: {}",indirect.table);
                    }
                    
                    match req.url.query{
                        None => response::create_response(status::BadRequest, "no specific record specified"),
                        Some(ref param) => {
                            let filter = queryst::parse(param);
                            match filter{
                                Err(e) => response::create_response(status::BadRequest, "error parsing params"),
                                Ok(filter) => {
                                    println!("filter: {:?}", filter);
                                    assert!(filter.is_object());
                                    let obj = filter.as_object().unwrap();
                                    for (key, value) in obj.iter() {
                                        println!("{}: {}", key, value);//make sure each key is a column of a table
                                        if table.has_column_name(key){
                                            println!("valid column: {}", key);
                                        }else{
                                            println!("table {} has no column {}", table, key);
                                            return response::create_response(status::BadRequest, 
                                                &format!("table {} has no column {}", table, key))
                                        }
                                    }
                                    response::create_response(status::Ok, "Table detail..")
                                },
                            }
                        },
                    }
                },
            }
        },
    }
    
}

pub fn set_db_url(req: &mut Request) -> IronResult<Response> {
    println!("Setting db url...");
    let mut content = String::new();
    req.body.read_to_string(&mut content).unwrap();
    println!("content: {}",content);
    let db_url = content;
    SessionHash::set_db_url(req, &db_url);
    return response::create_response(status::Ok, "Ok");
}