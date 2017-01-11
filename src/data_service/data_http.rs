use iron::status;
use router::Router;
use iron::prelude::*;
use rustc_serialize::json;

use rustorm::database::{Database, DatabaseDev};
use rustorm::dao::DaoResult;
use rustorm::database::DbError;
use rustorm::table::Column;
use data_service;
use inquerest;
use iron::status::Status;
use global::Context;

pub fn http_data_query(req: &mut Request) -> IronResult<Response> {

    let table: String = match req.extensions.get::<Router>().unwrap().find("table") {
        Some(table) => table.to_owned(),
        None => return Ok(Response::with((Status::BadRequest, "No table specified"))),
    };
    let param: Option<String> = match req.url.query {
        Some(ref param) => Some(param.to_owned()),
        None => None,
    };

    let iq = match param {
        Some(param) => Some(inquerest::parse(&param).unwrap()),
        None => None,
    };
    let mut context = Context::new(req);
    let json = data_service::data_json::json_data_query(&mut context, &table, iq);
    match json {
        Ok(json) => Ok(Response::with((Status::Ok, json))),
        Err(json) => Ok(Response::with((Status::BadRequest, json))),
    }
}
