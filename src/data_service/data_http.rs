use iron::status;
use router::Router;
use iron::prelude::*;
use rustc_serialize::json::{self};

use rustorm::database::{Database,DatabaseDev};
use rustorm::dao::{DaoResult};
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
//use queryst;
use data_service;
use inquerest;
use from_query::FromQuery;
use iron::status::Status;
use global::{self, GlobalPools};

pub fn http_data_query(req: &mut Request)->IronResult<Response>{
	
	let table:String = match req.extensions.get::<Router>().unwrap().find("table"){
		Some(table) => table.to_owned(),
		None => return Ok(Response::with((Status::BadRequest, "No table specified")))
	};
	let param:Option<String> = match req.url.query{
		Some(ref param) => Some(param.to_owned()),
		None => None
	};

	let globals = GlobalPools::from_request(req);
	let iq = match param{
		Some(param) => Some(inquerest::query(&param).unwrap()),
		None => None
	};
	let db_url = global::get_db_url(req).unwrap();
	let platform = globals.write().unwrap().get_connection(&db_url).unwrap();
	let json = data_service::data_json::json_data_query(globals, &db_url, platform.as_dev(), platform.as_ref(), &table, iq);
	Ok(Response::with((Status::Ok, json)))
}

pub fn http_insert(req: &mut Request)->IronResult<Response>{
	let table:String = match req.extensions.get::<Router>().unwrap().find("table"){
		Some(table) => table.to_owned(),
		None => return Ok(Response::with((Status::BadRequest, "No table specified")))
	};
	let param:Option<String> = match req.url.query{
		Some(ref param) => Some(param.to_owned()),
		None => None
	};
	let mut body = String::new();
	match req.body.read_to_string(&mut body){
		Ok(x) => println!("read {} bytes",x),
		Err(e) => println!("Unable to read body")
	};
	println!("body: {}", body);
	let globals = GlobalPools::from_request(req);
	let db_url = global::get_db_url(req).unwrap();
	let platform = globals.write().unwrap().get_connection(&db_url).unwrap();
	let json = data_service::data_json::json_insert_data(globals, &db_url, platform.as_dev(), platform.as_ref(), &table, &body);
	Ok(Response::with((Status::Ok, json)))
}

pub fn http_update(req: &mut Request)->IronResult<Response>{
	let table:String = match req.extensions.get::<Router>().unwrap().find("table"){
		Some(table) => table.to_owned(),
		None => return Ok(Response::with((Status::BadRequest, "No table specified")))
	};
	let param:Option<String> = match req.url.query{
		Some(ref param) => Some(param.to_owned()),
		None => None
	};
	let mut body = String::new();
	match req.body.read_to_string(&mut body){
		Ok(x) => println!("read {} bytes",x),
		Err(e) => println!("Unable to read body")
	};
	println!("body: {}", body);
	let globals = GlobalPools::from_request(req);
	let iq = match param{
		Some(param) => inquerest::query(&param).unwrap(),
		None => return Ok(Response::with((Status::BadRequest, "Filter is required"))) 
	};
	let db_url = global::get_db_url(req).unwrap();
	let platform = globals.write().unwrap().get_connection(&db_url).unwrap();
	let json = data_service::data_json::json_update_data(globals, &db_url, platform.as_dev(), platform.as_ref(), &table, iq, &body);
	Ok(Response::with((Status::Ok, json)))
}

pub fn http_delete_query(req: &mut Request)->IronResult<Response>{
	
	let table:String = match req.extensions.get::<Router>().unwrap().find("table"){
		Some(table) => table.to_owned(),
		None => return Ok(Response::with((Status::BadRequest, "No table specified")))
	};
	let param:Option<String> = match req.url.query{
		Some(ref param) => Some(param.to_owned()),
		None => None
	};

	let globals = GlobalPools::from_request(req);
	let iq = match param{
		Some(param) => inquerest::query(&param).unwrap(),
		None => return Ok(Response::with((Status::BadRequest, "Filter is required"))) 
	};
	let db_url = global::get_db_url(req).unwrap();
	let platform = globals.write().unwrap().get_connection(&db_url).unwrap();
	let json = data_service::data_json::json_delete_query(globals, &db_url, platform.as_dev(), platform.as_ref(), &table, iq);
	Ok(Response::with((Status::Ok, json)))
}

