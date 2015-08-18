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
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
    let mut pool = ManagedPool::init(&url, 1);
    let db = pool.connect();
    match db{
        Ok(db) => {
            list_all_windows(db.as_dev());
        }
        Err(error) =>{
            println!("{}",error);
        }
    }
}


pub fn list_all_windows(db_dev:&DatabaseDev){
    let all_tables = generator::get_all_tables(db_dev);
    let windows = window::list_windows(&all_tables);
    
    let content = format!("{}",json::as_pretty_json(&windows));
    println!("{}", content);
    
    let filename = "window_list.json";
    match File::create(&filename){
        Err(why) => panic!("couldn't create file {}", filename),
        Ok(mut file) => {
            file.write_all(content.as_bytes());
            println!("Saved to {}",&filename);
        },
    };
}
