extern crate inquerest;

use inquerest::*;

fn main(){
    let q = query("age=lt.13&(student=eq.true|gender=eq.M)&group_by=sum(age),grade,gender&having=min(age)=gt.13&order_by=age.desc,height.asc&page=20&page_size=100&x=123&y=456");
    println!("{:#?}", q);
    println!("{:?}", q);
}