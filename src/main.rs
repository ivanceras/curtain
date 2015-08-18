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

mod window;
mod identifier;

pub struct AppDb;
impl Key for AppDb { type Value = ManagedPool; }


fn say_hello(req: &mut Request) -> IronResult<Response> {
    println!("Running send_hello handler, URL path: {}", req.url.path.connect("/"));
    Ok(Response::with((status::Ok, "This request was routed!")))
}

fn get_window(req: &mut Request) -> IronResult<Response> {
    let pool = req.get::<Read<AppDb>>().unwrap();
    let table_name = req.extensions.get::<Router>().unwrap().find("table");
    match table_name{
        Some(ref table_name) => {
            println!("table_name: {:?}", table_name);
            let db = pool.connect();
            match db {
                Ok(db) => {
                    match window::get_window(db.as_dev(), table_name){
                        Ok(window) => {
                            let encoded = json::encode(&window);
                            let mut response = Response::with((status::Ok, encoded.unwrap()));
                            response.headers.set(AccessControlAllowOrigin::Any);
                            return Ok(response)
                        },
                        Err(e) => {
                            let mut response = Response::with((status::BadRequest, e));
                            response.headers.set(AccessControlAllowOrigin::Any);
                            return Ok(response)
                        }
                    }
                },
                Err(e) => return Ok(Response::with((status::BadRequest, "Unable to connect to database")))
            }
            
            
        },
        None =>{
             return Ok(Response::with((status::BadRequest, "No table specified")))
        }
    }
}

fn main() {
    
    let url = get_db_url();
    let pool = ManagedPool::init(&url, 10);
    
    let mut router = Router::new();
    router
        .get("/hello", say_hello)
        .get("/window/:table", get_window);
        //.options("/window/:table", get_window);
    
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
    match env::var("DATABASE_URL") {
        Ok(val) => val,
        Err(_) => "postgres://postgres:p0stgr3s@localhost/bazaar_v6".to_string()
    }
}
