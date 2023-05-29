use deadpool_diesel::postgres::{Manager, Object};
use diesel::prelude::*;

pub fn establish_connection_pool (db_url: String) -> deadpool_diesel::Pool<Manager, Object> { // In theory I don't need the db_url anywhere else, so it should be fine to take ownership
    let manager = deadpool_diesel::postgres::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    deadpool_diesel::Pool::builder(manager)
        .build()
        .unwrap()
}