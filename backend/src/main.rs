use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
    response::Json,
    routing::get,
    Router,
};
use diesel::prelude::*;
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use versequest_be::{models::user::User, schema};

const PORT: u16 = 8080;
const ADDR: &str = "0.0.0.0";

type PoolAlias = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

struct DatabaseConnection(
    bb8::PooledConnection<'static, AsyncDieselConnectionManager<AsyncPgConnection>>,
);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    S: Send + Sync,
    PoolAlias: FromRef<S>,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PoolAlias::from_ref(state);

        let conn = pool.get_owned().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_diesel_async_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
    let pool = bb8::Pool::builder()
        .max_size(20)
        .build(config)
        .await
        .unwrap();

    let router = Router::new()
        .route("/User/list", get(get_users))
        .with_state(pool);

    axum::Server::bind(&format!("{ADDR}:{PORT}").parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();

    tracing::debug!("listening on {PORT}");
}

async fn get_users(
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let res = schema::users::table
        .select(User::as_select())
        .load(&mut conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(res))
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
