extern crate inquerest;

use inquerest::*;

fn main() {
	println!("{:?}",operand("description"));
	println!("{:?}",boolean("true"));
	println!("column name{:?}",column_name("description"));
	println!("column name{:?}",column_name("product.description"));
	println!("{:?}",function("sum(total)"));
	println!("eq: {:?}",equality("eq"));
	println!("neq: {:?}",equality("neq"));
	println!("lt: {:?}",equality("lt"));
	println!("lte: {:?}",equality("lte"));	
	println!("lteee: {:?}",equality("lteee"));	
	println!("gt: {:?}",equality("gt"));	
	println!("{:?}",equality("gte"));	
	println!("in: {:?}",equality("in"));	
	println!("not_in: {:?}",equality("not_in"));	
	println!("is_not: {:?}",equality("is_not"));	
	println!("like:{:?}",equality("like"));	
	println!("function:{:?}",function("min(age)"));	
	println!("condition:{:?}",condition("age=lt.13"));	
	println!("condition:{:?}",condition("(age=lt.13)"));	
	println!("direction:{:?}",direction("asc"));	
	println!("direction:{:?}",direction("desc"));	
	println!("order:{:?}",order("age.desc"));
	println!("order:{:?}",order("height.asc"));
	println!("connector:{:?}",connector("|"));
	println!("connector:{:?}",connector("&"));
	println!("\n filter1: {:?}",filter("student=eq.true"));
	println!("\n filter1: {:?}",filter("(student=eq.true)"));
	println!("\n filter1: {:?}",filter("((student=eq.true))"));
	println!("\n filter2: {:?}",filter("student=eq.true|gender=eq.M"));
	println!("\n filter2: {:?}",filter("(student=eq.true&age=lt.13)"));
	println!("\n filter3: {:?}",filter("(student=eq.true)&(gender=eq.M)"));
	println!("\n filter4: {:?}",filter("student=eq.true&student=eq.true"));
	println!("\n filter4: {:?}",filter("student=eq.true&student=eq.true&age=lt.13"));
	println!("\n filter5: {:?}",filter("(student=eq.true)|(student=eq.true)"));
	println!("\n filter6: {:?}",filter("(student=eq.true)|(student=eq.true&age=lt.13)"));
	println!("\n filter6: {:?}",filter("(student=eq.true|student=eq.true)&age=lt.13)"));
	println!("\n filter7: {:#?}",filter("(student=eq.true)|(student=eq.true)&(age=lt.13)"));
	
	assert_eq!(condition("age=lt.13"), condition("(age=lt.13)"))
	
}

