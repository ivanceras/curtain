use global::Context;
use lookup_service::lookup_api;
use rustc_serialize::json;
use iron::prelude::*;
use config;

pub fn json_get_lookup_data(context: &mut Context, table: &str) -> Result<String, String> {
    match lookup_api::get_lookup_data(context, table) {
        Ok(lookup_data) => {
            let json = if config::PRETTY_JSON {
                format!("{}", json::as_pretty_json(&lookup_data))
            } else {
                json::encode(&lookup_data).unwrap()
            };
            Ok(json)
        }
        Err(e) => Err(format!("{}", e)),
    }
}

pub fn json_get_lookup_tabs(context: &mut Context, table: &str) -> Result<String, String> {
    match lookup_api::get_lookup_tabs(context, table) {
        Ok(lookup_data) => {
            let json = if config::PRETTY_JSON {
                format!("{}", json::as_pretty_json(&lookup_data))
            } else {
                json::encode(&lookup_data).unwrap()
            };
            Ok(json)
        }
        Err(e) => Err(format!("{}", e)),
    }
}
