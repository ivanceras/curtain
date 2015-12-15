extern crate iron;
extern crate mount;
extern crate persistent;
extern crate router;
extern crate rustorm;
extern crate rustc_serialize;
extern crate codegenta;
extern crate rand;
#[macro_use]extern crate log;
extern crate env_logger;
extern crate unicase;
extern crate uuid;
extern crate queryst;
extern crate inquerest;
extern crate arm_rest;

use iron::status;
use router::Router;
use std::str::FromStr;
use std::env;
use iron::prelude::*;
use persistent::{Write};
use std::net::SocketAddrV4;
use std::net::Ipv4Addr;
use global::GlobalPools;
use iron::method::Method::*;
use iron::AfterMiddleware;
use unicase::UniCase;
use iron::headers;

mod window;
mod window_service;
mod global;
mod data_service;



fn say_hello(req: &mut Request) -> IronResult<Response> {
    println!("Running send_hello handler, URL path: {}", req.url.path.connect("/"));
    let mut response = Response::with((status::Ok, "This request was routed!"));
    Ok(response)
}

fn main() {
    env_logger::init().unwrap();
    info!("starting up");
    
    let mut router = Router::new();
    router
        .get("/", say_hello)
        .get("/window", window_service::http_list_window)
        .get("/window/:table", window_service::http_get_window)
        .get("/data/:table",data_service::data_http::http_get_data)
        .get("/data_query/:table",data_service::data_http::http_data_query)
        .get("/detail/:table",data_service::data_http::http_table_detail)
        ;
    let mut middleware = Chain::new(router);
    middleware.link(Write::<GlobalPools>::both(GlobalPools::new()));
	middleware.link_after(CORS);
    let host = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), get_server_port());
    println!("listening on http://{}", host);
    Iron::new(middleware).http(host).unwrap();
}

struct CORS;

impl AfterMiddleware for CORS {
    fn after(&self, _: &mut Request, mut res: Response) -> IronResult<Response> {
        res.headers.set(headers::AccessControlAllowOrigin::Any);
        res.headers.set(headers::AccessControlAllowHeaders(
                vec![
					UniCase("accept".to_owned()), UniCase("content-type".to_owned()),
					UniCase("db_url".to_owned()), UniCase("*".to_owned())
				]));
        res.headers.set(headers::AccessControlAllowMethods(
                vec![Get,Head,Post,Delete,Options,Put,Patch]));
        Ok(res)
    }
} 

fn get_server_port() -> u16 {
    let port_str = env::var("PORT").unwrap_or(String::new());
    FromStr::from_str(&port_str).unwrap_or(8181)
}

