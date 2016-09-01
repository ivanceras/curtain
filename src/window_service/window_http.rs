use iron::status;
use router::Router;
use iron::prelude::*;
use iron::headers::*;
use iron::status::Status;
use rustc_serialize::json;

use rustorm::database::DatabaseDev;
use rustorm::query::TableName;
use window_service::window::{self, Window};
use rustorm::table::Table;
use unicase::UniCase;
use global::GlobalPools;
use global;
use window_service;
use global::Context;

/// TODO: Find out the implications here when the Arc is locked and used while a request on the other threads are served.
/// Safe bet would be to lock the Arc and make sure the executing functions is quick to finish

/// or to pass the Arc Mutex and lock unly when needed
pub fn http_list_window(req: &mut Request) -> IronResult<Response> {
    let mut context = Context::new(req);
    let json = window_service::window_json::json_list_window(&mut context);
    match json {
        Ok(json) => Ok(Response::with((Status::Ok, json))),
        Err(json) => Ok(Response::with((Status::BadRequest, json))),
    }
}


/// http request
pub fn http_get_window(req: &mut Request) -> IronResult<Response> {
    let table = match req.extensions.get::<Router>().unwrap().find("table") {
        Some(table) => table.to_owned(),
        None => return Ok(Response::with((Status::BadRequest, "No table specified"))),
    };
    let mut context = Context::new(req);
    let json = window_service::window_json::json_get_window(&mut context, &table);
    match json {
        Ok(json) => Ok(Response::with((Status::Ok, json))),
        Err(json) => Ok(Response::with((Status::BadRequest, json))),
    }
}
