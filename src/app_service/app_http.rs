
use iron::status;
use router::Router;
use std::str::FromStr;
use std::env;
use iron::prelude::*;
use persistent::{Write, State};
use std::net::SocketAddrV4;
use std::net::Ipv4Addr;
use global::GlobalPools;
use iron::method::Method::*;
use iron::AfterMiddleware;
use unicase::UniCase;
use iron::headers;
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
use app_service::app_api::ParseError;
use app_service::app_api::TableFilter;
use app_service;



pub fn http_complex_query(req: &mut Request)->IronResult<Response>{
    let mut context = Context::new(req);
	let complex_query = extract_complex_query(req);
	match complex_query{
		Ok((ref main_table_filter, ref rest_table_filter)) => {
			let json = app_service::app_json::json_complex_query(&mut context, main_table_filter, rest_table_filter);
			Ok(Response::with((status::Ok, json)))
		}
		Err(e) => {
			Ok(Response::with((status::BadRequest, format!("{:?}", e))))
		}
	}
}

fn extract_complex_query(req: &mut Request)->Result<(TableFilter, Vec<TableFilter>), ParseError>{
	let main_table = req.extensions.get::<Router>().unwrap().find("main_table");
	match main_table{
		Some(main_table) => {
			let query = &req.url.query;
			let mut main_table_filter = TableFilter{
					table: main_table.to_owned(),
					filter: None
				};
				
			let mut rest_table_filter = vec![];

			if let &Some(ref query) = &req.url.query{
				let table_queries:Vec<&str> = query.split("/").collect();
				if table_queries.len() > 0{
					main_table_filter.filter = Some(table_queries[0].to_owned());
				};
				let rest_query:Vec<&&str> = table_queries.iter().skip(1).collect();
				for q in rest_query{
					let table_filter: Vec<&str> = q.split("?").collect();
					let table = if table_filter.len() > 0 {
						Some(table_filter[0])
					}else{None};

					let filter = if table_filter.len() > 1 {
						Some(table_filter[1].to_owned())
					}else{None};

					if let Some(tbl) = table{
						let table_filter = TableFilter{
							table: tbl.to_owned(),
							filter: filter,
						};
						rest_table_filter.push(table_filter);
					}

				}
			}
			Ok((main_table_filter, rest_table_filter))
		},
		None => {
			Err(ParseError::new("No main table specified"))
		}
	}
}

