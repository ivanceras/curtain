extern crate rustorm;
extern crate curtain;
extern crate codegenta;
extern crate rustc_serialize;

use curtain::window;
use std::fs::File;
use std::io::Write;
use rustorm::query::Query;
use rustorm::query::{Filter,Equality};
use rustorm::dao::{Dao,IsDao};
use rustorm::pool::ManagedPool;
use codegenta::generator;
use rustorm::database::DatabaseDev;
use rustc_serialize::json;


fn main(){
     let url = "postgres://postgres:p0stgr3s@localhost/adempiere380";
    let mut pool = ManagedPool::init(&url, 1);
    let db = pool.connect();
    match db{
        Ok(db) => {
            derive_all_windows(db.as_dev());
        }
        Err(error) =>{
            println!("{}",error);
        }
    }
}


pub fn derive_all_windows(db_dev:&DatabaseDev){
    let all_tables = generator::get_all_tables(db_dev);
    window::extract_windows(&all_tables);
}