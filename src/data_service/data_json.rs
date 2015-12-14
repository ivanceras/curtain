//! API for json calls such as API calls from nodejs
//! and calls from http request which most likely needs to use json format
use rustc_serialize::json::{self, Json};
use rustorm::database::Database;
use inquerest;
use data_service;

pub fn retrieve_data_from_query(db: &Database, table: &str, iq: &inquerest::Query)->String{
	let result = data_service::data_api::retrieve_data_from_query(db, table, iq);
	match result{
		Ok(result) => {
			json::encode(&result).unwrap()
		},
		Err(e) => {
			format!("Json error")
		}
	}
}
