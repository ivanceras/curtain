extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;

use iron::status;
use iron::{Iron, Request, Response, IronResult};
use mount::Mount;
use router::Router;
use staticfile::Static;
use std::path::Path;
use std::str::FromStr;
use std::env;

fn say_hello(req: &mut Request) -> IronResult<Response> {
    println!("Running send_hello handler, URL path: {}", req.url.path.connect("/"));
    Ok(Response::with((status::Ok, "This request was routed!")))
}

fn main() {
    let mut router = Router::new();
    router
        .get("/hello", say_hello);

    let mut mount = Mount::new();
    mount
        .mount("/", router)
        .mount("/docs/", Static::new(Path::new("target/doc/")))
        .mount("/client/", Static::new(Path::new("client")));

    //Iron::new(mount).http("127.0.0.1:3001").unwrap();
     Iron::new(mount).http(("0.0.0.0", get_server_port())).unwrap();
}

fn get_server_port() -> u16 {
    let port_str = env::var("PORT").unwrap_or(String::new());
    FromStr::from_str(&port_str).unwrap_or(8080)
}
