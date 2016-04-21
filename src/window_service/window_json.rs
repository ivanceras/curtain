use rustorm::query::TableName;
use window_service::window::{self, Window};
use rustorm::table::Table;
use window_service;
use global::Context;
use rustc_serialize::json;

pub fn json_get_window<'a>(context: &mut Context, table: &str) -> Result<String, String>{
	match window_service::window_api::retrieve_window(context, table){
		Ok(window) => {
			Ok(json::encode(&window).unwrap())
		},
		Err(e) => {
			Err(format!("{}",e))
		}
	}
}


pub fn json_list_window(context: &mut Context) -> Result<String, String> {
	match window_service::window_api::list_window(context){
		Ok(window_list) => {
			Ok(json::encode(&window_list).unwrap())
		},
		Err(e) => {
			Err(format!("{}",e))
		}
	}
}
