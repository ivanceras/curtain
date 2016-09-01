use global::Context;
use rustc_serialize::json;
use app_service;
use config;


pub fn json_complex_query(context: &mut Context,
                          main_table: &str,
                          url_query: &Option<String>)
                          -> Result<String, String> {
    match app_service::app_api::complex_query(context, main_table, url_query) {
        Ok(rest_data) => {
            let json = if config::PRETTY_JSON {
                format!("{}", json::as_pretty_json(&rest_data))
            } else {
                json::encode(&rest_data).unwrap()
            };
            Ok(json)
        }
        Err(e) => Err(format!("{:?}", e)),
    }

}
pub fn json_focused_record(context: &mut Context,
                           main_table: &str,
                           url_query: &Option<String>)
                           -> Result<String, String> {
    match app_service::app_api::focused_record(context, main_table, url_query) {
        Ok(rest_data) => {
            let json = if config::PRETTY_JSON {
                format!("{}", json::as_pretty_json(&rest_data))
            } else {
                json::encode(&rest_data).unwrap()
            };
            Ok(json)
        }
        Err(e) => Err(format!("{:?}", e)),
    }

}


pub fn json_update_data(context: &mut Context,
                        main_table: &str,
                        body: &str)
                        -> Result<String, String> {
    match app_service::app_api::update_data(context, main_table, body) {
        Ok(()) => Ok(format!("OK")),
        Err(e) => Err(format!("{:?}", e)),
    }

}
