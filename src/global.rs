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
use rustorm::database::DatabaseDev;
use std::sync::{Arc,Mutex, RwLock};
use std;


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
/*
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
    */


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
		let pool = ManagedPool::init(db_url,10).unwrap();
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

pub struct Context{
    pub db_url: String,
    arc: Arc<RwLock<GlobalPools>>,
    platform: Option<Platform>,
}

impl Context{

    pub fn new(req:&mut Request)->Context{
        let db_url = get_db_url(req).unwrap();
        let globals = GlobalPools::from_request(req);
        

        let context = Context{
                        db_url: db_url.into(),
                        arc: globals,
                        platform: None
                       };
        context
    }
    
    pub fn db_dev<'a>(&'a mut self)->Result<&'a DatabaseDev,DbError>{
        let platform = self.get_connection();
        self.platform = Some(platform.unwrap());
        Ok(self.platform.as_ref().unwrap().as_dev())
    }
    fn db_url(&self)->String{
        self.db_url.to_owned()
    }
    pub fn db<'a>(&'a mut self)->Result<&'a Database,DbError>{
        match self.ensure_has_connection(){
            Ok(_) => Ok(self.platform.as_ref().unwrap().as_ref()),
            Err(e) => Err(e)
        }
    }
    fn ensure_has_connection(&mut self)->Result<(),DbError>{
        if self.platform.is_none(){
            let platform = self.get_connection();
            match platform{
                Ok(platform) => {
                    self.platform = Some(platform);
                    Ok(())
                  },
                Err(e) => {
                    Err(e)
                }
            } 
        }
        else{
            Ok(())
        }
    }

	fn get_connection(&self)->Result<Platform,DbError>{
		let has_cache = self.arc.read().unwrap().has_cache(&self.db_url);// note: need to finish the borrow so that write will not dead lock 
		if has_cache {
			let platform = self.arc.read().unwrap().get_cache(&self.db_url).unwrap().get_connection(); 
			return Ok(platform);
		}
		else{
			{// this is important to finish scope of arc and allow further write/read
				let cache = Cache::new(&self.db_url);
				let ref mut globals = *self.arc.write().unwrap();
				let db_url = self.db_url.clone();
				globals.cache_map.insert(db_url.clone(), cache);
			}
			return self.get_connection();//try again
		}
	}

    pub fn cache_tables(&self, tables: Vec<Table>){
        let ref mut globals = *self.arc.write().unwrap();
        globals.cache_tables(&self.db_url, tables);
    
    }

    pub fn has_cached_tables(&self)->bool{
        let ref globals = *self.arc.read().unwrap();
        globals.has_cached_tables(&self.db_url)
    }

    pub fn get_cached_tables(&self)->Option<Vec<Table>>{
        let ref globals = *self.arc.read().unwrap();
        globals.get_cached_tables(&self.db_url)
    }

    pub fn has_cached_windows(&self)->bool{
        let ref globals = *self.arc.read().unwrap();
        globals.has_cached_windows(&self.db_url)
    }

    pub fn get_cached_windows(&self)->Option<Vec<Window>>{
        let ref globals = *self.arc.read().unwrap();
        globals.get_cached_windows(&self.db_url)
    }

    pub fn cache_windows(&self, windows: Vec<Window>){
        let ref mut globals = *self.arc.write().unwrap();
        globals.cache_windows(&self.db_url, windows);
    }
}


