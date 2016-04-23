use rustorm::query::TableName;
use window_service::window::{self, Window};
use rustorm::table::Table;
use window_service;
use global::Context;
use rustc_serialize::json;
use config;

pub fn json_get_window(context: &mut Context, table: &str) -> Result<String, String>{
	match window_service::window_api::retrieve_window(context, table){
		Ok(window) => {
            let json = if config::PRETTY_JSON {
                format!("{}", json::as_pretty_json(&window))
            }else {
                json::encode(&window).unwrap()
            };
			Ok(json)
		},
		Err(e) => {
			Err(format!("{}",e))
		}
	}
}


pub fn json_list_window(context: &mut Context) -> Result<String, String> {
	match window_service::window_api::list_window(context){
		Ok(window_list) => {
            let json = if config::PRETTY_JSON {
                format!("{}", json::as_pretty_json(&window_list))
            }else {
                json::encode(&window_list).unwrap()
            };
			Ok(json)
		},
		Err(e) => {
			Err(format!("{}",e))
		}
	}
}
