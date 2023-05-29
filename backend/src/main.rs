mod schema;

use axum::{
    http::StatusCode,
    Router,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use dotenvy::dotenv;
use versequest_be::establish_connection_pool;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

const PORT: u16 = 3000;
const IP: &str = "0.0.0.0";

#[tokio::main]
async fn main() {

    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();


    let db_url = match std::env::var("DATABASE_URL") {
        Ok(db_url) => db_url,
        Err(_) => panic!("DATABASE_URL must be set!")
    };

    // set up connection pool
    let db_pool = establish_connection_pool(db_url);
    tracing::debug!("The connection to the db was successful");

    // run the migrations on server startup
    let conn = db_pool.get().await.unwrap();
    conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
        .await
        .unwrap()
        .unwrap();
    tracing::debug!("Migrations ran successfully");

    drop(conn);

    // build our application with some routes
    let app = Router::new()
        // .route("/posts/list", get(list_users))
        // .route("/posts/create", post(create_user))
        .with_state(db_pool);

    let addr: String = format!("{IP}:{PORT}");

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    tracing::debug!("Listening on {addr}");
}

// Utility function for mapping any error into a `500 Internal Server Error`
// response.
// fn internal_error<E>(err: E) -> (StatusCode, String)
//     where
//         E: std::error::Error,
// {
//     (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
// }