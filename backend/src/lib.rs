pub mod models;
pub mod schema;
pub mod api;

use axum::http::StatusCode;
use deadpool_diesel::postgres::{Manager, Object};
// use diesel::prelude::*; todo: remove?

pub fn establish_connection_pool (db_url: String) -> deadpool_diesel::Pool<Manager, Object> { // In theory I don't need the db_url anywhere else, so it should be fine to take ownership
    let manager = Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    deadpool_diesel::Pool::builder(manager)
        .build()
        .unwrap()
}

// Utility function for mapping any error into a `500 Internal Server Error`
// response.
pub fn internal_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}