//! API for json calls such as API calls from nodejs
//! and calls from http request which most likely needs to use json format
use rustc_serialize::json::{self,Json};
use inquerest;
use data_service;
use rustorm::dao::Dao;
use global::Context;


pub fn json_data_query(context: &mut Context, table:&str, iq: Option<inquerest::Query>)->Result<String, String>{
	match data_service::data_api::retrieve_data_from_query(context, table, iq){
		Ok(result) => Ok(json::encode(&result).unwrap()),
		Err(e) => Err(format!("{}",e))
	}
}

