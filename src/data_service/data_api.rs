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

use inquerest;

/// retrieve table record based on the query
/// use the first record as the selected record
/// then pull out the detail from other tables too
/// this would join all other extension tables
/// this would only use the filters, order, and paging
/// from, joins, columns,grouping are ignored
pub fn retrieve_data_from_query(globals: Arc<RwLock<GlobalPools>>, db_url: &str, db_dev: &DatabaseDev, db: &Database, arg_table: &str, iquery: Option<inquerest::Query>)->Result<DaoResult, DbError>{	
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
				let filter = correct_filters(&table, &fil.transform());
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
		Ok(result) => Ok(result),
		Err(e) => Err(e)
	}
}

/// focused_table - the main table of the list
/// focused_record - selected record in the table list

fn retrieve_focused_record_detail(globals: Arc<RwLock<GlobalPools>>, focused_table: &str, focused_record: inquerest::Query,
tab_table: &str, tab_filters: inquerest::Query)->Result<DaoResult, DbError>{
	panic!("ongoing!");
}

/// remove selected records from the database
/// only filters from iquery is included into the delete query
/// TODO: How is cascade constraint deal with. Delete the other records as well?
/// TODO: correct the data types of the right value of the filter based on the the column data type
/// returns the number of deleted recordds
pub fn delete_records(globals: Arc<RwLock<GlobalPools>>, db_url: &str, db_dev: &DatabaseDev, db: &Database, arg_table: &str, iquery: inquerest::Query)->Result<usize, DbError>{
	println!("Deleting records..");
	let table = window_service::window_api::get_matching_table(globals, db_url, db_dev, arg_table).unwrap();
	let mut query = Query::delete();
	query.from(&table);
	for ref fil in &iquery.filters{
		let filter = fil.transform();
		let cfilter = correct_filters(&table, &filter);
		query.filters.push(cfilter);
	}
	let ret = query.execute(db);
	match ret{
		Ok(n) => Ok(n),
		Err(e) => Err(e)
	}
}

pub fn insert_records(globals: Arc<RwLock<GlobalPools>>, db_url: &str, db_dev: &DatabaseDev, db: &Database, arg_table: &str, dao_list: &Vec<Dao>)->Result<DaoResult, DbError>{
	let table = window_service::window_api::get_matching_table(globals, db_url, db_dev, arg_table).unwrap();
	let mut query = Query::insert();
	query.into_(&table);
	for col in &table.columns{
		query.column(&col.name);
	}
	for dao in dao_list{
		for col in &table.columns{
			let dao_value = dao.get_value(&col.name);
			let cvalue = correct_value_type(&dao_value, &col.data_type);
			query.add_value(&cvalue);

		}
	}
	query.return_all();

	println!("update query: {:#?}", query);
	let ret = query.retrieve(db);
	println!("ret: {:?}",ret);
	ret
}
/// update the records
/// returns the updated records
pub fn update_records(globals: Arc<RwLock<GlobalPools>>, db_url: &str, db_dev: &DatabaseDev, db: &Database, arg_table: &str, iquery: inquerest::Query, dao: &Dao)->Result<DaoResult, DbError>{
	let table = window_service::window_api::get_matching_table(globals, db_url, db_dev, arg_table).unwrap();
	let mut query = Query::update();
	query.from(&table);
	for ref fil in &iquery.filters{
		let filter = fil.transform();
		let cfilter = correct_filters(&table, &filter);
		query.filters.push(cfilter);
	}
	for col in table.columns{
		println!("col: {} = {}", col.name, dao.get_value(&col.name));
		let dao_value = dao.get_value(&col.name);
		let cvalue = correct_value_type(&dao_value, &col.data_type);
		query.set_value(&col.name, &cvalue);

	}
	query.return_all();

	println!("update query: {:#?}", query);
	let ret = query.retrieve(db);
	println!("ret: {:?}",ret);
	ret
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
fn correct_value_type(orig: &Value, to_type: &Type)->Value{
	println!("converting {:?} to {:?}", orig, to_type);
	if orig.get_type() == *to_type {
		return orig.clone();
	}
	if orig.get_type() == Type::None{
		return Value::None(to_type.clone())
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
				Value::String(ref s) => Value::Json(Json::from_str(s).unwrap()),
				_ => panic!("Unable to convert from other type {:?} to {:?}", orig, to_type)
			}
		},
		_ => panic!("other types not yet..")
	}	
} 


/// insert new records
/// update dirty records with
/// returns the inserted records
/// the updated records
fn save_records(globals: Arc<RwLock<GlobalPools>>, table: &str, original_records: DaoResult, dirty_records: DaoResult, new_records: DaoResult)->Result<(DaoResult, DaoResult), DbError>{
	panic!("ongoing!");
}
