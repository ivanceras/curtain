use iron::prelude::*;
use persistent::{Write};
use rand::{thread_rng, Rng};

use rustorm::pool::ManagedPool;
use iron::typemap::Key;
use window::Window;
use rustorm::table::Table;
use std::collections::BTreeMap;
use rustorm::pool::Platform;
use rustorm::database::DbError;


/// each db_url has its own connection pool, even when they are using the same db platform
/// each db_url has its own cached values
pub struct GlobalPools{
	pub pool: BTreeMap<String, ManagedPool>,//connection pool indexed by db_url 
	pub cache: BTreeMap<String, Cache>, //caches indexed by db_url
}

impl Key for GlobalPools{
	type Value = GlobalPools;
}

impl GlobalPools{
	
	/// initialize the pools
	pub fn new()->Self{
		GlobalPools{
			pool: BTreeMap::new(),
			cache: BTreeMap::new()
		}
	}


	
	/// extract the globals from the Request
	pub fn from_request<'a>(req: &'a Request)->Option<&'a Self>{
		match req.get_ref::<Write<GlobalPools>>(){
			Ok(ref globals) => {
				Some(&*globals.lock().as_ref().unwrap())
			},
			Err(e) => panic!("Error reading from persistent {:?}", e)
		}
	}
	
	pub fn get_cache(&self, db_url: &str)->Option<&Cache>{
		self.cache.get(db_url)
	}
	pub fn get_pool(&self, db_url: &str)->Option<&ManagedPool>{
		self.pool.get(db_url)
	}

    /// cache this window values to this db_url
	pub fn cache_windows(&mut self, db_url: &str, windows: Vec<Window>){
		let cache = self.cache.remove(db_url);
		match cache{
			Some(cache) => {
				let mut cache = cache.clone();
				cache.windows = windows;
				let ret = self.cache.insert(db_url.to_string(), cache);
				if ret.is_none(){
					println!("Cached!");
				}else{
					println!("not cached!");
				}

			},
			None => {
				let cache = Cache {windows: windows, tables: vec![]};
				self.cache.insert(db_url.to_string(), cache);
			}
		}
    }


	pub fn cache_tables(&mut self, db_url: &str, tables: Vec<Table>){
		let mut cache = self.cache.remove(db_url).unwrap();
		cache.tables = tables;
		let ret = self.cache.insert(db_url.to_owned(), cache);
		if ret.is_none(){
			println!("Cached!");
		}else{
			println!("not cached!");
		}
	}

}

/// items cached, unique for each db_url connection
#[derive(Clone)]
pub struct Cache{
    /// windows extraction is an expensive operation and doesn't change very often
    pub windows: Vec<Window>,
    /// tables extraction is an expensive operation and doesn't change very often
    pub tables: Vec<Table>,
}

// the db_url is stored in the headers
pub fn get_db_url(req: &Request)->Option<String>{
	let db_url: Option<&[Vec<u8>]> = req.headers.get_raw("db_url");
	match db_url{
		Some(db_url) => {
			let first = &db_url[0];
			let url = String::from_utf8(first.clone()).unwrap();
			Some(url)
		},
		None => None
	}
}


