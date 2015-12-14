extern crate inquerest;
extern crate arm_rest;

use inquerest::query;
use arm_rest::FromQuery;

fn main(){
	
	let iq = query("age=lt.13").unwrap();
	println!("{:?}",iq);
	let query  = iq.transform();
	println!("transform: {:?}", query);

}
