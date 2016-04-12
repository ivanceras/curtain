use global::Context;
use rustc_serialize::json;
use app_service;


pub fn json_complex_query(context: &mut Context, main_table: &str, url_query: &Option<String>)->String{
	match app_service::app_api::complex_query(context, main_table, url_query){
		Ok(rest_data) => {
			json::encode(&rest_data).unwrap()
		}
		Err(e) => {
			format!("{:?}",e)
		}
	}
	
}
