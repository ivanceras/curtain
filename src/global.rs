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


/// a list of managed pool for each db_url
pub struct DatabasePool{
    map: BTreeMap<String, ManagedPool>
}
impl Key for DatabasePool{
    type Value = DatabasePool;
}

impl DatabasePool{
    
    pub fn new()->Self{
        DatabasePool{map: BTreeMap::new()}
    }
    
    fn get_ref(&self, db_url: &str)->Option<&ManagedPool>{
        self.map.get(db_url)
    }
    
    fn set(&mut self, db_url :&str,pool: ManagedPool)->Option<ManagedPool>{
        self.map.insert(db_url.to_string(), pool)
    }
    
    fn connect(req: &mut Request, db_url: &str)->Result<Platform, DbError>{
        let db_pool = match req.get_mut::<Write<DatabasePool>>(){
            Ok(db_pool) => db_pool,
            Err(e) => panic!("Error reading db_pool {:?}", e)
        };
        let mut db_pool = db_pool.lock().unwrap();
        
        let platform = match db_pool.get_ref(&db_url){
            Some(ref mut pool) => {
                Some(pool.connect())
            },
            None => {
                None
            }
        };
        if platform.is_none(){
            let pool = ManagedPool::init(&db_url, 1); //give 10 initial connection
            match pool{
                Ok(pool) => {
                    let con = pool.connect();
                    db_pool.set(&db_url, pool);
                    return con;
                },
                Err(e) => Err(DbError::new(&format!("{}", e)))
            }
        }else{
            return platform.unwrap();
        }
    }
    
    pub fn get_connection(req: &mut Request)->Result<Platform, DbError>{
        let db_url = SessionHash::get_db_url(req);
        match db_url{
            Some(ref db_url) => {
                DatabasePool::connect(req, db_url)
            }, 
            None => {
                error!("No db_url supplied in the request.. cant create a database pool");
                Err(DbError::new("No db_url supplied in the request.. cant create a database pool"))
            }
        }
    }
}

/// a cache containing database pool, windows, tables
/// which can be accessed by different session as long as they have the same db_url
pub struct Cache{
    /// windows extraction is an expensive operation and doesn't change very often
    windows: Vec<Window>,
    /// tables extraction is an expensive operation and doesn't change very often
    tables: Vec<Table>,
}



/// a container for cached objects shared by session which have the same db_url, regardless wether they have different session_key
pub struct CachePool{
    /// key is db_url
    map: BTreeMap<String, Cache>
}

impl Key for CachePool{
    type Value = CachePool;
}

impl CachePool{
    
     pub fn new()->Self{
        CachePool{map: BTreeMap::new()}
    }
     
    /// get the connection pool with the connection url specified
    fn get_mut(&mut self, db_url: &str)->Option<&mut Cache>{
        self.map.get_mut(db_url)
    }
    
    /// get the connection pool with the connection url specified
    fn get_ref(&self, db_url: &str)->Option<&Cache>{
        self.map.get(db_url)
    }
    
    fn set(&mut self, db_url: &str, cache: Cache)->Option<Cache>{
        self.map.insert(db_url.to_string(), cache)
    }
    
    /// put the extracted windows into this cache, accessible by the db_url key
    pub fn cache_windows(req: &mut Request, windows: Vec<Window>){
        let db_url = SessionHash::get_db_url(req);
        match db_url{
            Some(db_url) => {
                let cache_pool = match req.get_mut::<Write<CachePool>>(){
                    Ok(cache_pool) => cache_pool,
                    Err(e) => panic!("Error reading cache pool {:?}", e)
                };
                let mut cache_pool = cache_pool.lock().unwrap();
                
                let has_cache = match cache_pool.get_mut(&db_url){
                    Some(ref mut cache) => {
                        cache.windows = windows.clone();
                        true
                    },
                    None => {
                        debug!("no session to cache to.... creating a new one");
                        false
                    }
                };
                if !has_cache {
                    let cache = Cache{
                                tables: vec![],
                                windows: windows,
                            };
                    cache_pool.set(&db_url, cache);
                }
            }, 
            None => {
                error!("No db_url supplied in the request")
            }
        }
    }
    /// put the extracted tables into this cache, accessible by the db_url key
    pub fn cache_tables(req: &mut Request, tables: Vec<Table>){
        let db_url = SessionHash::get_db_url(req);
        match db_url{
            Some(db_url) => {
                let cache_pool = match req.get_mut::<Write<CachePool>>(){
                    Ok(cache_pool) => cache_pool,
                    Err(e) => panic!("Error reading cache pool {:?}", e)
                };
                let mut cache_pool = cache_pool.lock().unwrap();
                
                let has_cache = match cache_pool.get_mut(&db_url){
                    Some(ref mut cache) => {
                        cache.tables = tables.clone();
                        true
                    },
                    None => {
                        debug!("no session to cache to.... creating a new one");
                        false
                    }
                };
                if !has_cache {
                    let cache = Cache{
                                tables: tables,
                                windows: vec![],
                            };
                    cache_pool.set(&db_url, cache);
                }
            }, 
            None => {
                error!("No db_url supplied in the request");
            }
        }
        
    }
    
    pub fn get_cached_tables(req: &mut Request)->Vec<Table>{
        let db_url = SessionHash::get_db_url(req);
        match db_url{
            Some(db_url) => {
                let cache_pool = match req.get_ref::<Write<CachePool>>(){
                    Ok(cache_pool) => cache_pool,
                    Err(e) => panic!("Error reading cache pool {:?}", e)
                };
                match cache_pool.lock().unwrap().get_ref(&db_url){
                    Some(ref cache) => {
                        cache.tables.clone()
                    },
                    None => {
                        warn!("No cached tables.. try getting from the db");
                        vec![]
                    }
                }
            }, 
            None => {
                error!("No db_url specified");
                vec![]
            }
        }
        
    }
    
    pub fn get_cached_windows(req: &mut Request)->Vec<Window>{
        let db_url = SessionHash::get_db_url(req);
        match db_url{
            Some(db_url) => {
                let cache_pool = match req.get_ref::<Write<CachePool>>(){
                    Ok(cache_pool) => cache_pool,
                    Err(e) => panic!("Error reading cache pool {:?}", e)
                };
                match cache_pool.lock().unwrap().get_ref(&db_url){
                    Some(ref cache) => {
                        cache.windows.clone()
                    },
                    None => {
                        warn!("Now cached windows... try rebuilding from the tables");
                        vec![]
                    }
                }
            }, 
            None => {
                error!("No db_url specified");
                vec![]
            }
        }
        
    }
    
    
}


#[derive(Debug)]
#[derive(RustcEncodable,RustcDecodable)]
struct Param{
    session_key: Option<String>,
}

pub struct Session{
    session_key: String,
    db_url: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

/// How to clear the unused sessions
pub struct SessionHash{
    map: BTreeMap<String, Session>
}

impl Key for SessionHash{
    type Value = SessionHash;
}


impl SessionHash{
    
    pub fn new()->Self{
        SessionHash{map: BTreeMap::new()}
    }
    

    /// 20 character key
    fn generate_session_key()->String{
        let key:String = thread_rng().gen_ascii_chars().take(20).collect();
        key
    }
    
    fn get_mut(&mut self, key: &str)->Option<&mut Session>{
        self.map.get_mut(key)
    }
    fn get_ref(&self, key: &str)->Option<&Session>{
        self.map.get(key)
    }
    
    fn set(&mut self, key: &str, session: Session)->Option<Session>{
        self.map.insert(key.to_string(), session)
    }
    
    
    /// extract the session key from the request
    pub fn get_session_key(req: &Request)->Option<String>{
        let session_key: Option<&[Vec<u8>]> = req.headers.get_raw("session_key");
        match session_key{
            Some(session_key) => {
                let first = &session_key[0];
                let key = String::from_utf8(first.clone()).unwrap();
                Some(key)
            },
            None => None
        }
    }
    
    /// get the db url from headers
     pub fn get_db_url(req: &Request)->Option<String>{
        let db_url: Option<&[Vec<u8>]> = req.headers.get_raw("db_url");
        match db_url{
            Some(db_url) => {
                let first = &db_url[0];
                let key = String::from_utf8(first.clone()).unwrap();
                Some(key)
            },
            None => {
                error!("unable to get url from {:?}", db_url);
                None
            }
        }
    }
    
    /// get the database url used for this session
    /// TODO: seems like, if getting a reference other than Write cant get the shared value
    fn get_db_url_from_session(req: &mut Request)->Option<String>{
       let key = SessionHash::get_session_key(req);
        match key{
            Some(key) => {
                let session_hash = req.get_ref::<Write<SessionHash>>().unwrap();
                match session_hash.lock().unwrap().get_ref(&key){
                    Some(ref session) => {
                        match &session.db_url{
                            &Some(ref db_url) => Some(db_url.to_string()),
                            &None => None
                        }
                    },
                    None => {None}
                }
            },
            None => {
                panic!("No session key found in the request")
            }
        }
    }
    
    /// start a new session if there none
    pub fn session_headers(req: &mut Request, resp: &mut Response){
        let session_key = SessionHash::get_session_key(req);
        let value = match session_key{
            Some(session_key) => {
                println!("It has an existing session");
                session_key
            },
            None => {
                let session_key = SessionHash::new_session(req);
                println!("No existing session");
                //setting the sesion key in the request
                req.headers.set_raw("session_key", vec![session_key.clone().into_bytes()]);
                session_key
            }
        };
        println!("Setting session_key");
        resp.headers.set_raw("session_key", vec![value.into_bytes()])
    }
    
    /// create a new session
    pub fn new_session(req: &mut Request)->String{
        let session_key =  SessionHash::generate_session_key();
        let session_hash = match req.get::<Write<SessionHash>>(){
            Ok(x) => x,
            Err(e) => panic!("Error reading session hash {:?}", e)
        };
        let session = Session{
                    session_key: session_key.to_string(),
                    db_url: None,
                    username: None,
                    password: None,
                };
        session_hash.lock().unwrap().set(&session_key, session);
        session_key
    }
    
    
    /// set an existing session with a new db_url
    pub fn set_db_url(req: &mut Request, db_url: &str){
        let key = SessionHash::get_session_key(req);
        match key{
            Some(key) => {
                let session_hash = req.get_mut::<Write<SessionHash>>().unwrap();
                match session_hash.lock().unwrap().get_mut(&key){
                    Some(ref mut session) => {
                        session.db_url = Some(db_url.to_string());
                    },
                    None => {}
                }
            },
            None => {
                panic!("No session key found in the request")
            }
        }
                
    }

}


