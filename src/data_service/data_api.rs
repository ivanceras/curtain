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

	let ret = query.retrieve(db);
	match ret{
		Ok(result) => Ok(SerDaoResult::from_dao_result(result)),
		Err(e) => Err(e)
	}
}


/*
/// retrieve the record detail
/// the filter query is required
/// where the record id is determined
/// record_id is the primary_key value of the record,
/// or unique value when no primary key is defined
/// or all columns when no primary_key nor unique indexes defined
/// extension tabs has 1:1 record
/// paging will be applied to has_many and indirect has_many record
/// FIXME: how to express filter for the has_many and indirect has_many records
/// Resolve: There should be 1 request for each tab presentation
/// Realization: Use table.column to apply only on their respect tables
/// Use tab.paging for paging on their respective tables as well.
/// Question 1: Does paging of has_many tabs will have to request using retrieve record_detail or to their respective table specific api
/// table.focused=table.u_column1=eq.value1&table.u_column2=eq.value2&table2.focused=table2.p_column1=eq.value1&table.page=10&table.page_size=100

fn retrieve_record_detail(db: &Database, table: &str, iquery: inquerest::Query)->Result<SerDaoResult, DbError>{

}
*/
