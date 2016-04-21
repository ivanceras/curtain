extern crate rustorm;
extern crate curtain;


use rustorm::query::Query;
use rustorm::query::QueryBuilder;
use rustorm::query::Equality;
use rustorm::dao::{Dao, IsDao};
use rustorm::pool::ManagedPool;
use curtain::window_service::window_api;
use rustorm::table::Table;
use curtain::window_service::window::Tab;

fn main(){

	let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v8";
    let pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();
		
	let all_tables = window_api::get_all_tables_from_db(db.as_dev());
	let tally = window_api::get_table_tally(&all_tables);
	for (table, count) in tally{
		println!("{} {}", table.complete_name(), count);
	}
	let order_line = Table::get_table(&Some("bazaar".to_owned()), &"order_line".to_owned(), &all_tables);
	let orders = Table::get_table(&Some("bazaar".to_owned()), &"orders".to_owned(), &all_tables);

	println!("order_line: {:?}", order_line);
	let is_owned = order_line.is_owned_or_semi_owned(&all_tables);
	//println!("order line has one: {:?}", order_line.has_one(&all_tables));
	//println!("order line has many: {:?}", order_line.has_many(&all_tables));
	println!("order line is owned: {}", is_owned); 
	println!("order is owned: {}", orders.is_owned_or_semi_owned(&all_tables));

	let order_tab = Tab::detailed_from_table(&orders, &all_tables);
	println!("order tab: {:#?}", order_tab);
	let order_line_tab = Tab::detailed_from_table(&order_line, &all_tables);
	println!("order line tab: {:#?}", order_line_tab);
	let mut owned = 0;
	for ref table in &all_tables{
		if table.is_owned_or_semi_owned(&all_tables){
			owned += 1;
			let owner = table.referred_tables(&all_tables);
			let (col, owner_table) = owner[0];
			println!("{} owns {} ",owner_table.complete_name(),table.complete_name());
		}
		if table.is_owned_or_semi_owned(&all_tables){
			println!("SEMI OWNED ---> {}", table.complete_name());
		}
	}
	println!("owned tables: {}", owned);
}
	
