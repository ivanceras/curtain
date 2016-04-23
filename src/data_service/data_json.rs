//! API for json calls such as API calls from nodejs
//! and calls from http request which most likely needs to use json format
use rustc_serialize::json::{self, Json};
use rustorm::database::{Database,DatabaseDev};
use inquerest;
use data_service;
use global::GlobalPools;
use from_query::FromQuery;
use rustorm::dao::Dao;
use std::sync::{Arc,RwLock};
use global::Context;


pub fn json_data_query(context: &mut Context, table:&str, iq: Option<inquerest::Query>)->Result<String, String>{
	match data_service::data_api::retrieve_data_from_query(context, table, iq){
		Ok(result) => Ok(json::encode(&result).unwrap()),
		Err(e) => Err(format!("{}",e))
	}
}

