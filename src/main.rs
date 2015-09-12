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
extern crate uuid;

use iron::status;
use router::Router;
use std::str::FromStr;
use std::env;
use iron::prelude::*;
use persistent::{Write};
use std::net::SocketAddrV4;
use std::net::Ipv4Addr;
use global::SessionHash;
use global::CachePool;
use global::DatabasePool;


mod window;
mod window_service;
mod identifier;
mod global;
mod data_service;
mod response;



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
    
    let mut router = Router::new();
    router
        .get("/", say_hello)
        .get("/db_url", show_db_url)
        .get("/window", window_service::list_window)
        .options("/window", response::preflight)
        .get("/window/:table", window_service::get_window)
        .options("/window/:table", response::preflight)
        .get("/data/:table",data_service::get_data)
        .options("/data/:table",response::preflight)
        .get("/detail/:table",data_service::table_detail)
        .options("/detail/:table",response::preflight)
        .post("/db",data_service::set_db_url)
        ;
    let mut middleware = Chain::new(router);
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

