extern crate rustorm;
extern crate curtain;


use rustorm::query::Query;
use rustorm::query::QueryBuilder;
use rustorm::query::Equality;
use rustorm::dao::{Dao, IsDao};
use rustorm::pool::ManagedPool;
use curtain::window_service::window_api;

#[test]
fn test_order_line_is_owned(){

	let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v8";
    let pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();
		
	let all_tables = window_api::get_all_tables(db.as_dev());
	println!("all tables: {:#}", all_tables);
}
	
