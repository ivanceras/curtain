extern crate iron;
extern crate hyper;

use iron::prelude::*;
use iron::headers::*;
use iron::Url;
use std::net::IpAddr;
use std::net::SocketAddr;
use iron::method::Method;
use iron::typemap::TypeMap;
use std::str::FromStr;
use hyper::http::h1::HttpReader;
use iron::request::Body;
use std::io::prelude::*;
use std::net::TcpStream;
use hyper::buffer::BufReader;
use hyper::net::HttpStream;
use hyper::net::NetworkStream;
use hyper::net::HttpsStream;

fn main(){
    
    let url = Url::parse("http://localhost").unwrap();
    let ip = IpAddr::from_str("127.0.0.1").unwrap();
    let addr = SocketAddr::new(ip,8080);
    let mut stream = TcpStream::connect("127.0.0.1:34254").unwrap();
    let mut http_stream = HttpStream(stream);
    let mut https_stream = HttpsStream::Http(http_stream);

    let content = String::new();
    let mut bufreader = BufReader::new(&mut https_stream);
    let reader = HttpReader::EmptyReader(&mut bufreader);
    let body = Body::new(reader);

    let req = Request{
        url: url,
        remote_addr: addr.clone(),
        local_addr: addr.clone(),
        headers: Headers::new(),
        body: body,
        method: Method::Get,
        extensions: TypeMap::new()
   };

}
