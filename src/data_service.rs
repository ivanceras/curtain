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
use response;
use queryst;
use window_service;
use std::collections::BTreeMap;
use rustorm::table::{Table, Column};
use rustc_serialize::json::Json;
use rustorm::query::{Filter, Equality, Join, Modifier, ToTableName};
use rustorm::dao::Value;
use uuid::Uuid;

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

/// build a join query for the this extension table
/// filter_obj is used in the WHERE filter statement
/// extension_foreign will be used in the LEFT JOIN 
fn build_ext_join(db: &Database, filter_obj: &BTreeMap<String, Json>, table: &Table, ext_table: &Table)->Result<DaoResult, DbError>{
    let extension_foreign = ext_table.get_foreign_columns_to_table(table);//this will be used for left join
    println!("SELECT * FROM {}", ext_table.complete_name());
    let mut query = Query::enumerate_all();
        query.only_from(ext_table);
     
    println!("LEFT JOIN {}", table.complete_name());
    let mut column1 = vec![];
    let mut column2 = vec![];
    for ext_fc in extension_foreign{
        println!("ext fc: {}", ext_fc);
        let foreign = &ext_fc.foreign.as_ref().unwrap();
        println!("foreign: {:?}", foreign);
        column1.push(format!("{}.{}", ext_table.name, ext_fc.name));
        column2.push(format!("{}.{}", foreign.table, foreign.column));
    }
    
  let join = Join{
        modifier:Some(Modifier::LEFT),
        join_type:None,
        table_name: table.to_table_name(),
        column1:column1,
        column2:column2
    };
    query.join(join);
    
    for (key, value) in filter_obj.iter() {
        println!("{}: {}", key, value);//make sure each key is a column of a table
        if table.has_column_name(key){
            println!("valid column: {}", key);
            println!("WHERE {} = {}", key, value);
            let column = table.get_column(key).unwrap();
            assert!(value.is_string());
            let value_str = value.as_string().unwrap();
            let corrected_value = correct_data_type(&column, &value_str).unwrap();
            let key_column = format!("{}.{}",table.name, column.name);
            let filter = Filter::with_value(&key_column, Equality::EQ, corrected_value);
            query.add_filter(filter);
        }else{
            warn!("table {} has no column {}", table, key);
        }
    }
    let sql = query.build(db);
    println!("SQL: {}",sql);
    query.retrieve(db)
}

fn correct_data_type<'a>(column: &Column, value: &str)->Option<Value>{
    if column.data_type == "Uuid"{
        let uuid = Uuid::parse_str(value).unwrap();
        return Some(Value::Uuid(uuid))
    }
    else if column.data_type == "String" {
        return Some(Value::String(value.to_string()))
    }
    else{
        panic!("dont know how to convert to {}",column.data_type);
    }
    None
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
                    let param:Option<String> = {
                        match req.url.query{
                            None => None,
                            Some(ref param) => Some(param.to_string()),
                        }
                    };
                    match param{
                        None => response::create_response(status::BadRequest, "no specific record specified"),
                        Some(ref param) => {
                            let filter = queryst::parse(param);
                            match filter{
                                Err(e) => response::create_response(status::BadRequest, "error parsing params"),
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
                                            return response::create_response(status::BadRequest, 
                                                &format!("table {} has no column {}", table, key))
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
                                                let ext_result = build_ext_join(db.as_ref(), filter_obj, &table, &ext_table);
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
                                    
                                    response::create_response(status::Ok, &json::encode(&ext_results).unwrap())
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