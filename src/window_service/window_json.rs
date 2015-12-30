use iron::status;
use router::Router;
use iron::prelude::*;
use iron::headers::*;
use iron::status::Status;
use rustc_serialize::json::{self};
use codegenta::generator;
use std::sync::{Arc,RwLock};

use rustorm::database::DatabaseDev;
use rustorm::query::TableName;
use window_service::window::{self, Window};
use rustorm::table::Table;
use unicase::UniCase;
use global::GlobalPools;
use global;
use window_service;

pub fn json_get_window<'a>(globals: Arc<RwLock<GlobalPools>>, db_url: &str, db_dev: &DatabaseDev, table: &str) -> String{
	match window_service::window_api::retrieve_window(globals, db_url, db_dev, table){
		Ok(window) => {
			json::encode(&window).unwrap()
		},
		Err(e) => {
			format!("{}",e)
		}
	}
}


pub fn json_list_window(globals: Arc<RwLock<GlobalPools>>, db_url:&str, db_dev: &DatabaseDev) -> String {
	match window_service::window_api::list_window(globals, db_url, db_dev){
		Ok(window_list) => {
			json::encode(&window_list).unwrap()
		},
		Err(e) => {
			format!("{}",e)
		}
	}
}
