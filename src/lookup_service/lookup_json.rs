use global::Context;
use lookup_service::lookup_api;
use rustc_serialize::json;
use iron::prelude::*;

pub fn json_get_lookup_data(context: &mut Context, table: &str)->String{
	match lookup_api::get_lookup_data(context, table){
		Ok(lookup_data) => {
			json::encode(&lookup_data).unwrap()
		}
		Err(e) => format!("{}",e)
	}
}
