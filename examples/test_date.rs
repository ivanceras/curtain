extern crate chrono;
extern crate rustc_serialize;
use chrono::naive::date::NaiveDate;
use rustc_serialize::json;
fn main(){
    println!("testing dates..");    
    let d = NaiveDate::from_ymd(2015, 6, 3);
    
    println!("date: {:?}", d);
    println!("date: {}", &format!("{}",json::as_pretty_json(&d)));
    
    println!("from ce: {:?}", NaiveDate::from_num_days_from_ce_opt(730000))
    
}