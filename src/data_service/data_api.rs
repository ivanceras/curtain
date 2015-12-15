use iron::status;
use router::Router;
use iron::prelude::*;
use rustc_serialize::json::{self};

use rustorm::database::Database;
use rustorm::dao::{SerDaoResult, DaoResult};
use rustorm::database::DbError;
use rustorm::query::Query;
use std::io::Read;
use window_service;
use std::collections::BTreeMap;
use rustorm::table::{Table, Column};
use rustc_serialize::json::Json;
use rustorm::query::{Filter, Equality, Join, Modifier, ToTableName};
use rustorm::dao::Value;
use uuid::Uuid;
use queryst;
use from_query::{FromOrder, FromFilter, FromRange};

use inquerest;

pub fn retrieve_data(db: &Database, table: &str, page_size: usize)->Result<SerDaoResult, DbError>{
    let mut query = Query::select_all();
    let result = query.from(&table)
          .set_page_size(page_size)
          .retrieve(db);
    match result{
        Ok(result) => Ok(SerDaoResult::from_dao_result(result)),
        Err(e) => Err(e)
    }
}
/// retrieve table record based on the query
/// use the first record as the selected record
/// then pull out the detail from other tables too
/// this would join all other extension tables
/// this would only use the filters, order, and paging
/// from, joins, columns,grouping are ignored
pub fn retrieve_data_from_query(db: &Database, table: &str, iquery: Option<inquerest::Query>)->Result<SerDaoResult, DbError>{	
	let mut query = Query::select_all();
	// tables are gotten from window service
	// main table is used, while extension tables is left joined
	// has many tables are looped and retrieved for each record
	// while the for the extension tables,
	// the ref table is determined and used to the joined
	query.from_table(table);
	match iquery{
		Some(iquery) => {
			for ref fil in &iquery.filters{
				let filter = fil.transform();
				query.filters.push(filter);
			}
			for ref ord in &iquery.order_by{
				let order_by = ord.transform();
				query.order_by.push(order_by);
			}
			match &iquery.range{
				&Some(ref rng) => {
					let range = rng.transform();
					query.range = Some(range);
				},
				&None => {}
			};
		},
		None => ()
	};

	println!("query: {:?}", query);
	let ret = query.retrieve(db);
	println!("ret: {:#?}" ,ret);
	match ret{
		Ok(result) => Ok(SerDaoResult::from_dao_result(result)),
		Err(e) => Err(e)
	}
}

