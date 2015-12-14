use iron::status;
use router::Router;
use iron::prelude::*;
use rustc_serialize::json::{self};

use rustorm::database::Database;
use rustorm::dao::{SerDaoResult, DaoResult};
use rustorm::database::DbError;
use rustorm::query::Query;
use global::SessionHash;
use std::io::Read;
use global::DatabasePool;
use window_service;
use std::collections::BTreeMap;
use rustorm::table::{Table, Column};
use rustc_serialize::json::Json;
use rustorm::query::{Filter, Equality, Join, Modifier, ToTableName};
use rustorm::dao::Value;
use uuid::Uuid;
use queryst;
use data_service;
use inquerest;
use data_service::from_query::FromQuery;
use iron::status::Status;


pub fn data_query(req: &mut Request)->IronResult<Response>{
	let db = DatabasePool::get_connection(req).unwrap();
	let table_name = req.extensions.get::<Router>().unwrap().find("table").unwrap();
	let param = req.url.query.as_ref().unwrap();
	let iq = inquerest::query(&param).unwrap();
	//FIXME: do necessary security here to check validity of query params
	let query = iq.transform();
	println!("query:{:#?}",query);
	let result = data_service::data_json::retrieve_data_from_query(db.as_ref(), table_name, &iq);
	let  response = Response::with((Status::Ok, result));
	Ok(response)
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
                    let data = data_service::data_api::retrieve_data(db.as_ref(), table_name, page_size);
                    match data{
                        Ok(data) => {
                                let encoded = json::encode(&data);
                                return Ok(Response::with((status::Ok, encoded.unwrap())));
                            },
                        Err(e) => {
                                return Ok(Response::with((status::BadRequest, format!("{}",e))));
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
        return Ok(Response::with((status::BadRequest, "No table specified")))
    }
    let page_size = 20;//page will be sent in the range header
    println!("table: {:?}", arg_table);
    let db = DatabasePool::get_connection(req);
    match db{
        Err(e) => Ok(Response::with((status::BadRequest, "Unable to connect to Database"))),
        Ok(db) => {
            let arg_table = arg_table.unwrap();
            let window = window_service::retrieve_window_api(req, db.as_dev(), &arg_table);
            match window{
                Err(e) => Ok(Response::with((status::BadRequest, "No table with that name"))),
                Ok(window) => {
                    println!("query: {:?}", req.url.query);

                    println!("window: {} {}", window.name, window.table);
                    let table = window_service::get_matching_table(req, db.as_dev(), &window.table).unwrap();
                    let param:Option<String> = {
                        match req.url.query{
                            None => None,
                            Some(ref param) => Some(param.to_string()),
                        }
                    };
                    match param{
                        None => Ok(Response::with((status::BadRequest, "no specific record specified"))),
                        Some(ref param) => {
                            let filter = queryst::parse(param);
                            match filter{
                                Err(e) => Ok(Response::with((status::BadRequest, "error parsing params"))),
                                Ok(filter) => {
                                    println!("filter: {:?}", filter);
                                    assert!(filter.is_object());
                                    let filter_obj = filter.as_object().unwrap();
                                    for (key, value) in filter_obj.iter() {
                                        println!("{}: {}", key, value);//make sure each key is a column of a table
                                        if table.has_column_name(key){
                                            println!("valid column: {}", key);
                                        }else{
                                            println!("table {} has no column {}", table, key);
                                            return Ok(Response::with((status::BadRequest, 
                                                format!("table {} has no column {}", table, key))))
                                        }
                                    }
                                    let tab = window.tab.unwrap();
                                    let mut ext_results = vec![];
                                    for ext_tab in tab.ext_tabs.unwrap(){
                                        println!("ext_tab: {}",ext_tab.table);
                                        let ext_table = window_service::get_matching_table(req, db.as_dev(), &ext_tab.table);
                                        match ext_table{
                                            None => error!("No table for this extension tab?"),
                                            Some(ext_table) => {
                                                let ext_result = data_service::data_api::build_ext_join(db.as_ref(), filter_obj, &table, &ext_table);
                                                println!("ext_result: {:?}", ext_result);
                                                match ext_result{
                                                    Err(e) => {},
                                                    Ok(ext_result) => {
                                                        ext_results.push(ext_result);
                                                    }
                                                }
                                           }
                                        }
                                    }
                                    for has_many in tab.has_many_tabs.unwrap(){
                                        println!("has_many: {}",has_many.table);
                                    }
                                     for indirect in tab.has_many_indirect_tabs.unwrap(){
                                        println!("has_many_indirect_tabs: {}",indirect.table);
                                    }
                                    
                                    Ok(Response::with((status::Ok, json::encode(&ext_results).unwrap())))
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
    return Ok(Response::with((status::Ok, "Ok")));
}
