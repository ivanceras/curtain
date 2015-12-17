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

/// TODO: Find out the implications here when the Arc is locked and used while a request on the other threads are served.
/// Safe bet would be to lock the Arc and make sure the executing functions is quick to finish

/// or to pass the Arc Mutex and lock unly when needed
pub fn http_list_window(req: &mut Request) -> IronResult<Response> {
	let db_url = global::get_db_url(req).unwrap();
	let globals = GlobalPools::from_request(req);
	let platform = globals.write().unwrap().get_connection(&db_url).unwrap();
	let json = window_service::window_json::json_list_window(globals, &db_url, platform.as_dev());
	Ok(Response::with((Status::Ok, json)))
}


/// http request
pub fn http_get_window(req: &mut Request) -> IronResult<Response> {
    let table = match req.extensions.get::<Router>().unwrap().find("table"){
        Some(table) => table.to_owned(),
        None => return Ok(Response::with((Status::BadRequest, "No table specified"))),
    };
	let db_url = global::get_db_url(req).unwrap();
	let globals = GlobalPools::from_request(req);
	let platform = globals.write().unwrap().get_connection(&db_url).unwrap();
	let json = window_service::window_json::json_get_window(globals, &db_url, platform.as_dev(), &table);
	Ok(Response::with((Status::Ok, json)))
}
