//! API for json calls such as API calls from nodejs
//! and calls from http request which most likely needs to use json format
use rustc_serialize::json::{self, Json};
use rustorm::database::Database;
use inquerest;
use data_service;
use global::GlobalPools;
use from_query::FromQuery;
use std::sync::{Arc,RwLock};



pub fn json_data_query(globals: Arc<RwLock<GlobalPools>>, db_url: &str, table:&str, iq: Option<inquerest::Query>)->String{
	let platform = globals.write().unwrap().get_connection(db_url).unwrap();
	let db = platform.as_ref();
	match data_service::data_api::retrieve_data_from_query(db, table, iq){
		Ok(result) => json::encode(&result).unwrap(),
		Err(e) => format!("{}",e)
	}
}

