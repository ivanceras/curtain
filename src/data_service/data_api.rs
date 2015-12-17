use iron::status;
use router::Router;
use iron::prelude::*;
use rustc_serialize::json::{self};

use rustorm::database::{Database, DatabaseDev};
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
use global::Cache;
use std::sync::{Arc,RwLock};
use global::GlobalPools;

use inquerest;

/// retrieve table record based on the query
/// use the first record as the selected record
/// then pull out the detail from other tables too
/// this would join all other extension tables
/// this would only use the filters, order, and paging
/// from, joins, columns,grouping are ignored
pub fn retrieve_data_from_query(globals: Arc<RwLock<GlobalPools>>, db_url: &str, db_dev: &DatabaseDev, db: &Database, arg_table: &str, iquery: Option<inquerest::Query>)->Result<SerDaoResult, DbError>{	
	let mut query = Query::select_all();
	let table = window_service::window_api::get_matching_table(globals, db_url, db_dev, arg_table).unwrap();
	// tables are gotten from window service
	// main table is used, while extension tables is left joined
	// has many tables are looped and retrieved for each record
	// while the for the extension tables,
	// the ref table is determined and used to the joined
	query.from(&table);
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

	let ret = query.retrieve(db);
	match ret{
		Ok(result) => Ok(SerDaoResult::from_dao_result(result)),
		Err(e) => Err(e)
	}
}

/// focused_table - the main table of the list
/// focused_record - selected record in the table list

fn retrieve_focused_record_detail(globals: Arc<RwLock<GlobalPools>>, focused_table: &str, focused_record: inquerest::Query,
tab_table: &str, tab_filters: inquerest::Query)->Result<SerDaoResult, DbError>{
	
	panic!("ongoing!");
}

