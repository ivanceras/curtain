use iron::status;
use router::Router;
use iron::prelude::*;
use iron::headers::*;
use persistent::{Read};
use rustc_serialize::json::{self};

use rustorm::database::Database;
use rustorm::dao::{DaoResult, SerDaoResult};
use rustorm::database::DbError;
use rustorm::query::Query;
use global::AppDb;

pub fn retrieve_data(db: &Database, table: &str, page_size: usize)->Result<SerDaoResult, DbError>{
    let mut query = Query::select_all();
    let result = query.from(&table)
          .set_page_size(page_size)
          .retrieve(db);
    match result{
        Ok(result) => Ok(SerDaoResult::from_dao_result(result)),
        Err(e) => Err(e)
    }
}

pub fn get_data(req: &mut Request) -> IronResult<Response> {
    let pool = req.get::<Read<AppDb>>().unwrap();
    let table_name = req.extensions.get::<Router>().unwrap().find("table");
    let page_size = 20;
    println!("query: {:?}", req.url.query);
    match table_name{
        Some(ref table_name) => {
            println!("table_name: {:?}", table_name);
            let db = pool.connect();
            match db {
                Ok(db) => {
                    let data = retrieve_data(db.as_ref(), table_name, page_size);
                    match data{
                        Ok(data) => {
                                let encoded = json::encode(&data);
                                let mut response = Response::with((status::Ok, encoded.unwrap()));
                                response.headers.set(AccessControlAllowOrigin::Any);
                                return Ok(response)                                
                            },
                        Err(e) => {
                                let mut response = Response::with((status::BadRequest, format!("{}",e)));
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