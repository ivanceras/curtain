use rustorm::pool::ManagedPool;
use iron::typemap::Key;

pub struct AppDb;
impl Key for AppDb { type Value = ManagedPool; }