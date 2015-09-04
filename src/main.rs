extern crate iron;
extern crate mount;
extern crate persistent;
extern crate router;
extern crate staticfile;
extern crate rustorm;
extern crate rustc_serialize;
extern crate codegenta;
extern crate queryst;
extern crate rand;
#[macro_use]extern crate log;
extern crate env_logger;
extern crate unicase;

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
//use global::AppDb;
use global::SessionHash;
use global::CachePool;
use global::DatabasePool;


mod window;
mod window_service;
mod identifier;
mod global;
mod data_service;



fn say_hello(req: &mut Request) -> IronResult<Response> {
    println!("Running send_hello handler, URL path: {}", req.url.path.connect("/"));
    let mut response = Response::with((status::Ok, "This request was routed!"));
    SessionHash::session_headers(req, &mut response);
    Ok(response)
}

fn show_db_url(req: &mut Request) -> IronResult<Response> {
    let db_url = SessionHash::get_db_url(req);
    let text = format!("db_url: {:?}", db_url);
    let mut response = Response::with((status::Ok, text));
    SessionHash::session_headers(req, &mut response);
    Ok(response)
}



fn main() {
    env_logger::init().unwrap();
    info!("starting up");
//    let url = get_db_url();
//    println!("using: {}",url);
//    let pool = ManagedPool::init(&url, 10);
    
    let mut router = Router::new();
    router
        .get("/", say_hello)
        .get("/db_url", show_db_url)
        .get("/window", window_service::list_window)
        .options("/window", window_service::preflight)
        .get("/window/:table", window_service::get_window)
        .options("/window/:table", window_service::preflight)
        .get("/data/:table",data_service::get_data)
        .options("/data/:table",window_service::preflight)
        .post("/db",data_service::set_db_url)
        ;
    let mut middleware = Chain::new(router);
//    middleware.link(Read::<AppDb>::both(pool));  
    middleware.link(Write::<DatabasePool>::both(DatabasePool::new()));
    middleware.link(Write::<SessionHash>::both(SessionHash::new()));
    middleware.link(Write::<CachePool>::both(CachePool::new()));
    let host = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), get_server_port());
    println!("listening on http://{}", host);
    Iron::new(middleware).http(host).unwrap();
}

fn get_server_port() -> u16 {
    let port_str = env::var("PORT").unwrap_or(String::new());
    FromStr::from_str(&port_str).unwrap_or(8080)
}

//fn get_db_url()->String{
//    let default = "postgres://postgres:p0stgr3s@localhost/bazaar_v7";
//    match env::var("DATABASE_URL") {
//        Ok(val) => val,
//        Err(_) => default.to_string()
//    }
//}
