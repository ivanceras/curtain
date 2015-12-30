//! API for json calls such as API calls from nodejs
//! and calls from http request which most likely needs to use json format
use rustc_serialize::json::{self, Json};
use rustorm::database::{Database,DatabaseDev};
use inquerest;
use data_service;
use global::GlobalPools;
use from_query::FromQuery;
use rustorm::dao::Dao;
use std::sync::{Arc,RwLock};



pub fn json_data_query(globals: Arc<RwLock<GlobalPools>>, db_url: &str, db_dev: &DatabaseDev, db: &Database, table:&str, iq: Option<inquerest::Query>)->String{
	match data_service::data_api::retrieve_data_from_query(globals, db_url, db_dev, db, table, iq){
		Ok(result) => json::encode(&result).unwrap(),
		Err(e) => format!("{}",e)
	}
}

pub fn json_delete_query(globals: Arc<RwLock<GlobalPools>>, db_url: &str, db_dev: &DatabaseDev, db: &Database, table:&str, iq: inquerest::Query)->String{
	match data_service::data_api::delete_records(globals, db_url, db_dev, db, table, iq){
		Ok(result) => json::encode(&result).unwrap(),
		Err(e) => format!("{}",e)
	}
}

pub fn json_update_data(globals: Arc<RwLock<GlobalPools>>, db_url: &str, db_dev: &DatabaseDev, db: &Database, table:&str, iq: inquerest::Query, body: &str)->String{
	let dao: Dao = Dao::from_str_one(body).unwrap();
	println!("dao: {:#?}", dao);
	match data_service::data_api::update_records(globals, db_url, db_dev, db, table, iq, &dao){
		Ok(result) => json::encode(&result).unwrap(),
		Err(e) => format!("{}",e)
	}
}

pub fn json_insert_data(globals: Arc<RwLock<GlobalPools>>, db_url: &str, db_dev: &DatabaseDev, db: &Database, table:&str, body: &str)->String{
	let dao: Vec<Dao> = Dao::from_str(body).unwrap();
	println!("dao: {:#?}", dao);
	match data_service::data_api::insert_records(globals, db_url, db_dev, db, table, &dao){
		Ok(result) => json::encode(&result).unwrap(),
		Err(e) => format!("{}",e)
	}
}
