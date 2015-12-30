extern crate iron;
extern crate persistent;
extern crate router;
extern crate uuid;
extern crate rustorm;
extern crate rustc_serialize;
extern crate chrono;
extern crate rand;
extern crate unicase;



use std::env;
use std::net::SocketAddrV4;
use std::net::Ipv4Addr;
use iron::prelude::*;
use iron::status;
use router::Router;
use persistent::{Write,Read};
use rustorm::pool::ManagedPool;
use iron::headers;
use iron::method::Method::*;
use iron::AfterMiddleware;
use unicase::UniCase;


// http://localhost:8181?age=lt.13&student=eq.true|gender=eq.M&group_by=sum(age),grade,gender&having=min(age)=gt.13&order_by=age.desc,height.asc&x=123&y=456
fn index(req: &mut Request) -> IronResult<Response> {
    let powered_by:String = match env::var("POWERED_BY") {
        Ok(val) => val,
        Err(_) => "Iron".to_string()
    };
    println!("{:#?}" ,req);
    let message = format!("Powered by: {}", powered_by);
    Ok(Response::with((status::Ok, message)))
}


fn main() {
    let mut router = Router::new();
    router.get("/", index);
    
    let mut middleware = Chain::new(router);
        
    let host = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8181);
    println!("listening on http://{}", host);
    Iron::new(middleware).http(host).unwrap();
}
