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
use data_service;
use inquerest;
use from_query::FromQuery;
use iron::status::Status;
use global::{self, GlobalPools};
use global::Context;

pub fn http_data_query(req: &mut Request)->IronResult<Response>{
	
	let table:String = match req.extensions.get::<Router>().unwrap().find("table"){
		Some(table) => table.to_owned(),
		None => return Ok(Response::with((Status::BadRequest, "No table specified")))
	};
	let param:Option<String> = match req.url.query{
		Some(ref param) => Some(param.to_owned()),
		None => None
	};

	let iq = match param{
		Some(param) => Some(inquerest::query(&param).unwrap()),
		None => None
	};
    let mut context = Context::new(req);
	let json = data_service::data_json::json_data_query(&mut context, &table, iq);
	Ok(Response::with((Status::Ok, json)))
}

