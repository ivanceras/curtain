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

	let arc = GlobalPools::from_request(req);
	let mut globals = arc.lock().unwrap();
	let iq = match param{
		Some(param) => Some(inquerest::query(&param).unwrap()),
		None => None
	};
	let db_url = global::get_db_url(req).unwrap();
	let json = data_service::data_json::json_data_query(&mut globals, &db_url, &table, iq);
	Ok(Response::with((Status::Ok, json)))
}

