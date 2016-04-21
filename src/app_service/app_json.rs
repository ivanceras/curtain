use global::Context;
use rustc_serialize::json;
use app_service;


pub fn json_complex_query(context: &mut Context, main_table: &str, url_query: &Option<String>)->Result<String, String>{
	match app_service::app_api::complex_query(context, main_table, url_query){
		Ok(rest_data) => {
			Ok(json::encode(&rest_data).unwrap())
		}
		Err(e) => {
			Err(format!("{:?}",e))
		}
	}
	
}


pub fn json_update_data(context: &mut Context, main_table: &str, body: &str)->Result<String, String>{
	match app_service::app_api::update_data(context, main_table, body){
		Ok(()) => {
		    Ok(format!("OK"))	
		}
		Err(e) => {
			Err(format!("{:?}",e))
		}
	}
	
}
