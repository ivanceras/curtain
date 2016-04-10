
use iron::prelude::*;
use iron::status::Status;
use router::Router;
use global::Context;
use lookup_service;

pub fn http_get_lookup_data(req: &mut Request)->IronResult<Response>{	
    let table = match req.extensions.get::<Router>().unwrap().find("table"){
        Some(table) => table.to_owned(),
        None => return Ok(Response::with((Status::BadRequest, "No table specified"))),
    };
	let mut context = Context::new(req);
	let json = lookup_service::lookup_json::json_get_lookup_data(&mut context, &table);
	Ok(Response::with((Status::Ok, json)))
}
