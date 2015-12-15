use iron::status;
use router::Router;
use iron::prelude::*;
use iron::headers::*;
use iron::status::Status;
use rustc_serialize::json::{self};
use codegenta::generator;

use rustorm::database::DatabaseDev;
use rustorm::query::TableName;
use window_service::window::{self, Window};
use rustorm::table::Table;
use unicase::UniCase;
use global::GlobalPools;
use global;
use window_service;

pub fn json_get_window<'a>(globals: &'a mut GlobalPools, db_url: &str, table: &str) -> String{
	let platform = globals.get_connection(db_url).unwrap();
	match window_service::window_api::retrieve_window(globals, db_url, platform.as_dev(), table){
		Ok(window) => {
			json::encode(&window).unwrap()
		},
		Err(e) => {
			format!("{}",e)
		}
	}
}


pub fn json_list_window(globals: &mut GlobalPools, db_url:&str) -> String {
	let platform = globals.get_connection(db_url).unwrap();
	match window_service::window_api::list_window(globals, db_url, platform.as_dev()){
		Ok(window_list) => {
			json::encode(&window_list).unwrap()
		},
		Err(e) => {
			format!("{}",e)
		}
	}
}
