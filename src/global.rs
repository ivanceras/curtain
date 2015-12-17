use iron::prelude::*;
use persistent::{Write, State};
use rand::{thread_rng, Rng};

use rustorm::pool::ManagedPool;
use iron::typemap::Key;
use window_service::window::Window;
use rustorm::table::Table;
use std::collections::BTreeMap;
use rustorm::pool::Platform;
use rustorm::database::DbError;
use rustorm::database::Database;
use std::sync::{Arc,Mutex, RwLock};



pub struct GlobalPools{
	pub cache_map: BTreeMap<String, Cache>, //caches indexed by db_url
}

impl Key for GlobalPools{
	type Value = GlobalPools;
}

impl GlobalPools{
	
	/// initialize the pools
	pub fn new()->Self{
		GlobalPools{
			cache_map: BTreeMap::new()
		}
	}

	pub fn from_request(req: &mut Request)->Arc<RwLock<Self>>{
		let global = req.get::<State<GlobalPools>>().unwrap();
		global
	}
	
	pub fn has_cache(&self, db_url: &str)->bool{
		self.cache_map.contains_key(db_url)
	}
	pub fn has_cached_tables(&self, db_url: &str)->bool{
		match self.get_cache(db_url){
			Some(cache) => cache.tables.is_some(),
			None => false
		}
	}
	
	pub fn get_cache(&self, db_url: &str)->Option<&Cache>{
		self.cache_map.get(db_url)
	}
	pub fn get_cached_tables(&self, db_url: &str)->Option<Vec<Table>>{
		match self.get_cache(db_url){
			Some(cache) => {
				match cache.tables{
					Some(ref tables) => Some(tables.clone()),
					None => None
				}
			}
			None => None
		}
	}

	pub fn has_cached_windows(&self, db_url: &str)->bool{
		match self.get_cache(db_url){
			Some(cache) => cache.windows.is_some(),
			None => false
		}
	}
	pub fn get_cached_windows(&self, db_url: &str)->Option<Vec<Window>>{
		match self.get_cache(db_url){
			Some(cache) => {
				match cache.windows{
					Some(ref windows) => Some(windows.clone()),
					None => None
				}
			}
			None => None
		}
	}
    /// cache this window values to this db_url
	pub fn cache_windows(&mut self, db_url: &str, windows: Vec<Window>){
		if self.has_cache(db_url) {
			let mut cache =  self.cache_map.remove(db_url).unwrap();
			cache.set_windows(windows);
			self.cache_map.insert(db_url.to_owned(), cache);
		}
		else{
			let mut cache = Cache::new(db_url);
			cache.set_windows(windows);
			self.cache_map.insert(db_url.to_owned(), cache);
		}
    }

	pub fn get_connection(&mut self, db_url: &str)->Result<Platform, DbError>{
		if self.has_cache(db_url){
			let platform = self.get_cache(db_url).unwrap().get_connection();
			Ok(platform)
		}else{
			let cache = Cache::new(db_url);
			self.cache_map.insert(db_url.to_owned(), cache);
			//try again
			self.get_connection(db_url)
		}
	}


	pub fn cache_tables(&mut self, db_url: &str, tables: Vec<Table>){
		if self.has_cache(db_url){
			let mut cache = self.cache_map.remove(db_url).unwrap();
			cache.set_tables(tables);
			self.cache_map.insert(db_url.to_owned(), cache);
		}else{
			let mut cache = Cache::new(db_url);
			cache.set_tables(tables);
			self.cache_map.insert(db_url.to_owned(), cache);
		}
	}

}

/// items cached, unique for each db_url connection
pub struct Cache{
	/// connections are cached here as well
	pub managed_pool: ManagedPool,
    /// windows extraction is an expensive operation and doesn't change very often
	/// None indicates, that nothing is cached yet, empty can be indicated as cached
    pub windows: Option<Vec<Window>>,
    /// tables extraction is an expensive operation and doesn't change very often
    pub tables: Option<Vec<Table>>,
}

impl Cache{
	
	fn new(db_url:&str)->Self{
		let pool = ManagedPool::init(db_url, 10).unwrap();
		Cache{
			managed_pool: pool,
			windows: None,
			tables: None,
		}
	}

	fn set_windows(&mut self, windows: Vec<Window>){
		self.windows = Some(windows);
	}
	fn set_tables(&mut self, tables: Vec<Table>){
		self.tables = Some(tables);
	}
	
	pub fn get_connection(&self)->Platform{
		self.managed_pool.connect().unwrap()
	}
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


