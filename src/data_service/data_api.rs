use iron::status;
use router::Router;
use iron::prelude::*;
use rustc_serialize::json::{self};

use rustorm::database::{Database, DatabaseDev};
use rustorm::dao::{DaoResult};
use rustorm::database::DbError;
use rustorm::query::Query;
use std::io::Read;
use window_service;
use std::collections::BTreeMap;
use rustorm::table::{Table, Column};
use rustc_serialize::json::Json;
use rustorm::query::{Filter, Condition, Equality, Join, Modifier, ToTableName, NullsWhere};
use rustorm::dao::Value;
use uuid::Uuid;
use from_query::{FromOrder, FromFilter, FromRange};
use global::Cache;
use std::sync::{Arc,RwLock};
use global::GlobalPools;
use rustorm::query::Operand;
use rustorm::dao::{Dao, Type};
use chrono::datetime::DateTime;
use std::str::FromStr;
use rustorm::query::ToSourceField;
use global::Context;
use window_service::window_api;
use inquerest;
use validator::DbElementValidator;
use config;

/// retrieve table record based on the query
/// use the first record as the selected record
/// then pull out the detail from other tables too
/// this would join all other extension tables
/// this would only use the filters, order, and paging
/// from, joins, columns,grouping are ignored
pub fn retrieve_data_from_query(context: &mut Context, arg_table: &str, iquery: Option<inquerest::Query>)->Result<DaoResult, DbError>{	
	let mut query = Query::select_all();
	let table = window_service::window_api::get_matching_table(context, arg_table).unwrap();
	// tables are gotten from window service
	// main table is used, while extension tables is left joined
	// has many tables are looped and retrieved for each record
	// while the for the extension tables,
	// the ref table is determined and used to the joined
	let validator = DbElementValidator::from_context(context);
	query.from(&table);
	match iquery{
		Some(iquery) => {
			for ref fil in &iquery.filters{
				let filter = correct_filters(&table, &fil.transform(&validator));
				query.filters.push(filter);
			}
			for ref ord in &iquery.order_by{
				let order_by = ord.transform(&validator);
				query.order_by.push(order_by);
			}
			match &iquery.range{
				&Some(ref rng) => {
					let range = rng.transform();
					query.range = range;
				},
				&None => {
                    query.set_limit(config::default_page_size);
                }
			};
		},
		None => ()
	};
	let ret = query.retrieve(context.db().unwrap());
	match ret{
		Ok(result) => Ok(result),
		Err(e) => Err(e)
	}
}


/// correct filters to their concise type, according to their column data types
fn correct_filters(table: &Table, filter: &Filter)->Filter{
	let new_condition = correct_condition(table, &filter.condition);
	let mut new_filter = filter.clone();
	new_filter.condition = new_condition;
	let mut new_sub_filters = vec![];
	for sub in &filter.sub_filters{
		let new_sub = correct_filters(table, sub);
		new_sub_filters.push(new_sub);
	}
	new_filter.sub_filters =  new_sub_filters;
	new_filter
}


fn correct_condition(table: &Table, condition: &Condition)->Condition{
	match &condition.left{
		&Operand::ColumnName(ref col) => {
			let column = table.get_column(&col.column).unwrap();
			let ltype = column.data_type;
			match &condition.right{
				&Operand::Value(ref v) => {
					let cright = correct_value_type(v, &ltype);
					let mut new_cond = condition.clone();
					new_cond.right = Operand::Value(cright);
					new_cond
				},
				_ => panic!("only assuming right side of condition is value")
			}
		},
		_ => panic!("not caring about other operands for now"),

	}
}

/// original value convert to type
pub fn correct_value_type(orig: &Value, to_type: &Type)->Value{
	println!("converting {:?} to {:?}", orig, to_type);
	if orig.get_type() == *to_type {
		return orig.clone();
	}
	match *to_type{
		Type::Uuid => {
			match *orig{
				Value::String(ref s) => Value::Uuid(Uuid::parse_str(s).unwrap()),
				_ => panic!("Unable to convert from other type {:?} to {:?}", orig, to_type)
			}
		},
		Type::DateTime => {
			match *orig{
				Value::String(ref s) => Value::DateTime(DateTime::from_str(s).unwrap()),
				_ => panic!("Unable to convert from other type {:?} to {:?}", orig, to_type)
			}
		},
		Type::F64 => {
			match *orig{
				Value::U64(u) => Value::F64(u as f64),
				_ => panic!("Unable to convert from other type {:?} to {:?}", orig, to_type)
			}
		},
		Type::I32 => {
			match *orig{
				Value::U64(u) => Value::I32(u as i32),
				_ => panic!("Unable to convert from other type {:?} to {:?}", orig, to_type)
			}
		},
		Type::Json => {
			match *orig{
				Value::String(ref s) => {
					let json = Json::from_str(s).unwrap();
					Value::Json(json)
				},
				_ => panic!("Unable to convert from other type {:?} to {:?}", orig, to_type)
			}
		},
		_ => panic!("other types not yet..")
	}	
} 


