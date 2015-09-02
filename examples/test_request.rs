extern crate iron;
extern crate mount;
extern crate persistent;
extern crate router;
extern crate staticfile;
extern crate rustorm;
extern crate rustc_serialize;
extern crate codegenta;
extern crate queryst;

use iron::status;
use mount::Mount;
use router::Router;
use staticfile::Static;
use std::path::Path;
use std::str::FromStr;
use std::env;
use rustorm::pool::ManagedPool;
use iron::prelude::*;
use iron::headers::*;
use iron::typemap::Key;
use persistent::{Write,Read};
use rustc_serialize::json::{self,ToJson};
use std::net::SocketAddrV4;
use std::net::Ipv4Addr;
use queryst::parse;

#[derive(Debug)]
#[derive(RustcEncodable,RustcDecodable)]
struct Param{
    session_id: Option<String>,
}

fn say_hello(req: &mut Request) -> IronResult<Response> {
    let query = req.url.query.clone();
    println!("Running send_hello handler, URL path: {:#?}", query);
    if query.is_some(){
        let query = query.unwrap();
        let params = parse(&query).unwrap();
        println!("params {:?}", params);
        let parsed:Param = json::decode(&format!("{}",params)).unwrap();
        println!("parsed: {:?}", parsed);
        let session_key = parsed.session_id.unwrap();
        println!("session_key: {:?}", session_key);
    }
    Ok(Response::with((status::Ok, "This request was routed!")))
}



fn main() {
    
    let mut router = Router::new();
    router
        .get("/", say_hello)
        ;
    
    let mut middleware = Chain::new(router);
    let host = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), get_server_port());
    println!("listening on http://{}", host);
    Iron::new(middleware).http(host).unwrap();
}

fn get_server_port() -> u16 {
    let port_str = env::var("PORT").unwrap_or(String::new());
    FromStr::from_str(&port_str).unwrap_or(8080)
}

