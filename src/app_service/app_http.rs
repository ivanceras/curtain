use iron::status;
use router::Router;
use iron::prelude::*;
use iron::headers::*;
use iron::status::Status;

use inquerest;
use global;
use global::Context;
use window_service::window_api;
use validator::DbElementValidator;
use from_query::FromQuery;
use rustorm::query::Query;
use inquerest as iq;
use rustorm::dao::Dao;
use rustorm::dao::DaoResult;
use rustorm::table::Table;
use rustorm::query::Equality;
use rustorm::query::{Filter,Join, JoinType, Modifier, Condition, Operand, Connector};
use rustorm::query::{TableName,ToTableName};
use rustorm::query::source::ToSourceField;
use rustorm::query::column_name::{ToColumnName,ColumnName};
use rustc_serialize::json;
use app_service::app_api::TableFilter;
use app_service;
use rustc_serialize::json::{Json};
use error::ParseError;
use std::io::Read;


/// example: http://localhost:8181/app/bazaar.product?price=gt.100.012e-10&order_by=product.seq_no&limit=10&focused=3/category?category.name=eq.accessories&order_by=name.asc.nullsfirst&focused=0
pub fn http_complex_query(req: &mut Request)->IronResult<Response>{
    let mut context = Context::new(req);
	match extract_params(req){
		Ok( (main_table, url_query) ) => {
			let json = app_service::app_json::json_complex_query(&mut context, &main_table, &url_query);
			Ok(Response::with((status::Ok, json)))
		}
		Err(e) => {
			Ok(Response::with((status::BadRequest, format!("{:?}", e))))
		}
	}
}

pub fn http_update_data(req: &mut Request)->IronResult<Response>{
    let mut context = Context::new(req);
	let main_table = req.extensions.get::<Router>().unwrap().find("main_table");
	let mut body = String::new(); 
	req.body.read_to_string(&mut body);

	match main_table{
		Some( main_table ) => {
            let json = app_service::app_json::json_update_data(&mut context, &main_table, &body);
			Ok(Response::with((status::Ok, json)))
		}
		None => {
			Ok(Response::with((status::BadRequest, "No main table specified")))
		}
	}
}

fn extract_params(req: &mut Request)->Result<(String, Option<String>),ParseError>{
	let main_table = req.extensions.get::<Router>().unwrap().find("main_table");
    if let Some(main_table) = main_table{
        let query = req.url.query.to_owned();
        return Ok((main_table.to_owned(), query));
    }
    Err(ParseError::new("no main table specified"))
}


