use iron::status;
use router::Router;
use iron::prelude::*;
use rustc_serialize::json::{self};

use rustorm::database::Database;
use rustorm::dao::{SerDaoResult, DaoResult};
use rustorm::database::DbError;
use rustorm::query::Query;
use global::SessionHash;
use std::io::Read;
use global::DatabasePool;
use response;
use window_service;
use std::collections::BTreeMap;
use rustorm::table::{Table, Column};
use rustc_serialize::json::Json;
use rustorm::query::{Filter, Equality, Join, Modifier, ToTableName};
use rustorm::dao::Value;
use uuid::Uuid;
use queryst;
use data_service::from_query::FromQuery;

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
pub fn retrieve_data_from_query(db: &Database, table: &str, iquery: &inquerest::Query)->Result<DaoResult, DbError>{	
	let query:Query = iquery.transform();
	panic!("not yet");

}

/// build a join query for the this extension table
/// filter_obj is used in the WHERE filter statement
/// extension_foreign will be used in the LEFT JOIN 
pub fn build_ext_join(db: &Database, filter_obj: &BTreeMap<String, Json>, table: &Table, ext_table: &Table)->Result<DaoResult, DbError>{
    let extension_foreign = ext_table.get_foreign_columns_to_table(table);//this will be used for left join
    println!("SELECT * FROM {}", ext_table.complete_name());
    let mut query = Query::enumerate_all();
        query.only_from(ext_table);
     
    println!("LEFT JOIN {}", table.complete_name());
    let mut column1 = vec![];
    let mut column2 = vec![];
    for ext_fc in extension_foreign{
        println!("ext fc: {}", ext_fc);
        let foreign = &ext_fc.foreign.as_ref().unwrap();
        println!("foreign: {:?}", foreign);
        column1.push(format!("{}.{}", ext_table.name, ext_fc.name));
        column2.push(format!("{}.{}", foreign.table, foreign.column));
    }
    
  let join = Join{
        modifier:Some(Modifier::LEFT),
        join_type:None,
        table_name: table.to_table_name(),
        column1:column1,
        column2:column2
    };
    query.join(join);
    
    for (key, value) in filter_obj.iter() {
        println!("{}: {}", key, value);//make sure each key is a column of a table
        if table.has_column_name(key){
            println!("valid column: {}", key);
            println!("WHERE {} = {}", key, value);
            let column = table.get_column(key).unwrap();
            assert!(value.is_string());
            let value_str = value.as_string().unwrap();
            let corrected_value = correct_data_type(&column, &value_str).unwrap();
            let key_column = format!("{}.{}",table.name, column.name);
            let filter = Filter::with_value(&key_column, Equality::EQ, corrected_value);
            query.add_filter(filter);
        }else{
            warn!("table {} has no column {}", table, key);
        }
    }
    let sql = query.build(db);
    println!("SQL: {}",sql);
    query.retrieve(db)
}

fn correct_data_type<'a>(column: &Column, value: &str)->Option<Value>{
    if column.data_type == "Uuid"{
        let uuid = Uuid::parse_str(value).unwrap();
        return Some(Value::Uuid(uuid))
    }
    else if column.data_type == "String" {
        return Some(Value::String(value.to_string()))
    }
    else{
        panic!("dont know how to convert to {}",column.data_type);
    }
    None
}
