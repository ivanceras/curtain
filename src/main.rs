#![feature(plugin)]
#![feature(question_mark)]
extern crate iron;
extern crate mount;
extern crate persistent;
extern crate router;
extern crate rustorm;
extern crate rustc_serialize;
extern crate rand;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate env_logger;
extern crate unicase;
extern crate uuid;
extern crate inquerest;
extern crate chrono;

use iron::status;
use router::Router;
use std::str::FromStr;
use std::env;
use iron::prelude::*;
use persistent::State;
use std::net::SocketAddrV4;
use std::net::Ipv4Addr;
use global::GlobalPools;
use iron::method::Method::*;
use iron::AfterMiddleware;
use unicase::UniCase;
use iron::headers;

mod global;
mod data_service;
mod from_query;
mod validator;
mod app_service;
mod window_service;
mod lookup_service;
mod error;
mod config;



fn say_hello(req: &mut Request) -> IronResult<Response> {
    println!("Running send_hello handler, URL path: {}",
             req.url.path.connect("/"));
    let mut response = Response::with((status::Ok, "This request was routed!"));
    Ok(response)
}

fn main() {
    //env_logger::init().unwrap();
    match log4rs::init_file("config/log4rs.yaml", Default::default()){
        Ok(_) => {println!("Loggin initiated..");}
        Err(e) => {println!("Something wrong in loggin.. {:?}", e);}
    }
    info!("starting up");
    warn!("warning example...");
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    println!("Curtain v{}", VERSION);

    let mut router = Router::new();
    router.get("/", say_hello);
    router.get("/window", window_service::window_http::http_list_window);
    router.get("/window/:table",
               window_service::window_http::http_get_window);
    router.get("/data/:table", data_service::data_http::http_data_query);
    router.get("/app/:main_table",
               app_service::app_http::http_complex_query);
    router.get("/app/focus/:main_table",
               app_service::app_http::http_focused_record);
    router.post("/app/:main_table", app_service::app_http::http_update_data);
    router.get("/lookup_data/:main_table",
               lookup_service::lookup_http::http_get_lookup_data);
    router.get("/lookup_tabs/:main_table",
               lookup_service::lookup_http::http_get_lookup_tabs);//get the lookup tabs for each lookup on this main_table
    router.delete("/cache", global::http_reset_cache);
    router.get("/connection", global::http_can_db_url_connect);

    let mut middleware = Chain::new(router);
    middleware.link(State::<GlobalPools>::both(GlobalPools::new()));
    middleware.link_after(CORS);
    let host = get_server_host();
    let listen = Iron::new(middleware).http(host);
    match listen{
        Ok(listen) => {
                println!("listensing on {:?}", listen)
            }
        Err(e) => {
                println!("Initialization error {}", e)
            }
    }

}


struct CORS;

impl AfterMiddleware for CORS {
    fn after(&self, req: &mut Request, mut res: Response) -> IronResult<Response> {
        let path = req.url.path.connect("/");
        warn!("warning has the request been served? {}", path);
        error!("an error has occured {}", path);
        trace!("tracing..{}",path);
        debug!("Debugging... {}",path);
        res.headers.set(headers::AccessControlAllowOrigin::Any);
        res.headers.set(headers::AccessControlAllowHeaders(vec![UniCase("accept".to_owned()),
                                                                UniCase("content-type"
                                                                    .to_owned()),
                                                                UniCase("db_url".to_owned()),
                                                                UniCase("*".to_owned())]));
        res.headers
            .set(headers::AccessControlAllowMethods(vec![Get, Head, Post, Delete, Options, Put,
                                                         Patch]));
        Ok(res)
    }
}


fn get_server_port() -> u16 {
    let port_str = env::var("PORT").unwrap_or(String::new());
    FromStr::from_str(&port_str).unwrap_or(3224)
}
#[cfg(feature = "webserver")]
fn get_server_host()-> SocketAddrV4 {
    SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), get_server_port())
}

/// when compiling for stand-alone, port will be decided by the OS, by sending arg 0
/// and the host will be 127.3.3.2 as opposed to common 127.0.0.1
#[cfg(feature = "standalone")]
fn get_server_host()-> SocketAddrV4 {
    SocketAddrV4::new(Ipv4Addr::new(127,0 ,0, 1), 0)
}

