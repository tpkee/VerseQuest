use axum::{
    routing::{get},
    http::StatusCode,
    Json, Router,
};

const PORT: u16 = 8080;
const ADDR: &str = "0.0.0.0";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root));

    axum::Server::bind(&format!("{ADDR}:{PORT}").parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    tracing::debug!("listening on {PORT}");
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}