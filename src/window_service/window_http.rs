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


pub fn http_list_window(req: &mut Request) -> IronResult<Response> {
	let db_url = global::get_db_url(req).unwrap();
	let arc = GlobalPools::from_request(req);
	let mut globals = arc.lock().unwrap();
	let json = window_service::window_json::json_list_window(&mut globals, &db_url);
	Ok(Response::with((Status::Ok, json)))
}


/// http request
pub fn http_get_window(req: &mut Request) -> IronResult<Response> {
    let table = match req.extensions.get::<Router>().unwrap().find("table"){
        Some(table) => table.to_owned(),
        None => return Ok(Response::with((Status::BadRequest, "No table specified"))),
    };
	let db_url = global::get_db_url(req).unwrap();
	let arc = GlobalPools::from_request(req);
	let mut globals = arc.lock().unwrap();
	let json = window_service::window_json::json_get_window(&mut globals, &db_url, &table);
	Ok(Response::with((Status::Ok, json)))
}
