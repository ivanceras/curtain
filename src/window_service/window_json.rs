use rustorm::query::TableName;
use window_service::window::{self, Window};
use rustorm::table::Table;
use window_service;
use global::Context;
use rustc_serialize::json;

pub fn json_get_window<'a>(context: &mut Context, table: &str) -> String{
	match window_service::window_api::retrieve_window(context, table){
		Ok(window) => {
			json::encode(&window).unwrap()
		},
		Err(e) => {
			format!("{}",e)
		}
	}
}


pub fn json_list_window(context: &mut Context) -> String {
	match window_service::window_api::list_window(context){
		Ok(window_list) => {
			json::encode(&window_list).unwrap()
		},
		Err(e) => {
			format!("{}",e)
		}
	}
}
