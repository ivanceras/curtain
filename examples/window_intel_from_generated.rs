extern crate rustorm;
extern crate curtain;

use rustorm::codegen;
use curtain::window;
use rustorm::database::DatabaseDev;
use rustorm::db::postgres::Postgres;

fn main(){
    let pg:Result<Postgres,&str> = Postgres::new("postgres://postgres:p0stgr3s@localhost/bazaar_v6");
    match pg{
        Ok(pg) => {
            derive_all_windows(&pg);
        }
        Err(error) =>{
            println!("{}",error);
        }
    }
}


pub fn derive_all_windows<T:DatabaseDev>(db_dev:&T){
    let all_tables = rustorm::gen::is_table::get_all_tables();
    window::extract_windows(&all_tables);
}