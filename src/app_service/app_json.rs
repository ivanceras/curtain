

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
use app_service::app_api::TableFilter;
use app_service;


pub fn json_complex_query(context: &mut Context, main_table_filter: &TableFilter, rest_table_filter: &Vec<TableFilter>)->String{
	match app_service::app_api::complex_query(context, main_table_filter, rest_table_filter){
		Ok(rest_data) => {
			json::encode(&rest_data).unwrap()
		}
		Err(e) => {
			format!("{:?}",e)
		}
	}
	
}
