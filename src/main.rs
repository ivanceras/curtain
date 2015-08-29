extern crate iron;
extern crate mount;
extern crate persistent;
extern crate router;
extern crate staticfile;
extern crate rustorm;
extern crate rustc_serialize;
extern crate codegenta;

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
use global::AppDb;

mod window;
mod window_service;
mod identifier;
mod global;
mod data_service;



fn say_hello(req: &mut Request) -> IronResult<Response> {
    println!("Running send_hello handler, URL path: {}", req.url.path.connect("/"));
    Ok(Response::with((status::Ok, "This request was routed!")))
}



fn main() {
    
    let url = get_db_url();
    println!("using: {}",url);
    let pool = ManagedPool::init(&url, 10);
    
    let mut router = Router::new();
    router
        .get("/", say_hello)
        .get("/window", window_service::list_window)
        .get("/window/:table", window_service::get_window)
        .get("/data/:table",data_service::get_data)
        ;
    
    let mut middleware = Chain::new(router);
    middleware.link(Read::<AppDb>::both(pool));  
    let host = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), get_server_port());
    println!("listening on http://{}", host);
    Iron::new(middleware).http(host).unwrap();
}

fn get_server_port() -> u16 {
    let port_str = env::var("PORT").unwrap_or(String::new());
    FromStr::from_str(&port_str).unwrap_or(8080)
}

fn get_db_url()->String{
    //let default = "postgres://postgres:p0stgr3s@localhost/bazaar_v7";
    //let default = "postgres://postgres:p0stgr3s@localhost:5432/device_farm_v2";
    let default = "postgres://postgres:p0stgr3s@http://45.55.7.231//bazaar_v7";
    match env::var("DATABASE_URL") {
        Ok(val) => val,
        Err(_) => default.to_string()
    }
}
