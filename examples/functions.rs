extern crate inquerest;

use inquerest::*;

fn main() {
	println!("function:{:?}",function("min(age)"));	
	println!("condition:{:?}",condition("min(grade)=gte.lee"));
}

