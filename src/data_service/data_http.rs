use iron::status;
use router::Router;
use iron::prelude::*;
use rustc_serialize::json::{self};

use rustorm::database::Database;
use rustorm::dao::{SerDaoResult, DaoResult};
use rustorm::database::DbError;
use rustorm::query::Query;
use std::io::Read;
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
use global::{self, GlobalPools};

pub fn data_query_http(req: &mut Request)->IronResult<Response>{
	let table = req.extensions.get::<Router>().unwrap().find("table").unwrap();
	let globals = GlobalPools::from_request(req).unwrap();
	let param = req.url.query.as_ref().unwrap();
	let iq = inquerest::query(&param).unwrap();
	let db_url = global::get_db_url(req).unwrap();
	data_query(&globals, &db_url, &table, &iq)
}
pub fn data_query(globals: &GlobalPools, db_url: &str, table:&str, iq: &inquerest::Query)->IronResult<Response>{
	//FIXME: do necessary security here to check validity of query params
	let query = iq.transform();
	println!("query:{:#?}",query);
	let pool = globals.get_pool(db_url).unwrap();
	let result = data_service::data_json::retrieve_data_from_query(pool.connect().unwrap().as_ref(), table, &iq);
	let  response = Response::with((Status::Ok, result));
	Ok(response)
}

pub fn get_data_http(req: &mut Request) -> IronResult<Response> {
    let globals = GlobalPools::from_request(req);
	let table_name = req.extensions.get::<Router>().unwrap().find("table");
    let page_size = 20;
    println!("query: {:?}", req.url.query);
	panic!("not yet");
}
pub fn get_data(globals: GlobalPools, db_url: &str, table: &str, page_size: usize) -> IronResult<Response> {
	let db = globals.get_pool(db_url).unwrap();
	let data = data_service::data_api::retrieve_data(db.connect().unwrap().as_ref(), table, page_size);
	match data{
		Ok(data) => {
				let encoded = json::encode(&data);
				Ok(Response::with((status::Ok, encoded.unwrap())))
		},
		Err(e) => {
				Ok(Response::with((status::BadRequest, format!("{}",e))))
		}
	}
}

/// extracts the details of the record,
/// it will extract the data of the tables that is linked to it

pub fn table_detail_http(req: &mut Request) -> IronResult<Response> {
	let table = req.extensions.get::<Router>().unwrap().find("table");
	let table = match table{
		Some(table) => table.to_owned(),
		None => panic!("no table specified!")
	};
	panic!("not yet here");	
}

pub fn table_detail(globals: &mut GlobalPools, db_url: &str, table: &str) -> IronResult<Response> {
    println!("Extracting table detail..");
    let page_size = 20;//page will be sent in the range header
	let db = globals.get_pool(db_url).unwrap() ;
	let window = window_service::retrieve_window_api(globals, db_url, db.connect().unwrap().as_dev(), &table);
	panic!("not yet!");
}

