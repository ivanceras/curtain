use iron::status;
use iron::prelude::*;
use iron::headers::*;
use iron::status::Status;

use unicase::UniCase;

pub fn create_response(status: Status, content: &str)->IronResult<Response>{
    let mut response = Response::with((status, content));
    response.headers.set(AccessControlAllowOrigin::Any);
    response.headers.set(AccessControlAllowHeaders(vec![
        UniCase("db_url".to_owned()),
        UniCase("*".to_owned()),
    ]));
    return Ok(response)
}

