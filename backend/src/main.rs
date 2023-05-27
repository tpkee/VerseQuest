use anyhow::{anyhow, Result};
use axum::{extract::State, routing::get, Router};
use shuttle_secrets::SecretStore;

#[derive(Clone)]
struct DatabaseSecrets {
    db_username: String,
    db_password: String,
    db_name: String,
    db_port: String,
    db_url: String,
}

#[shuttle_runtime::main]
async fn axum(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    let secrets = DatabaseSecrets {
        db_username: get_secret("DB_USERNAME", &secret_store),
        db_password: get_secret("DB_PASSWORD", &secret_store),
        db_name: get_secret("DB_NAME", &secret_store),
        db_port: get_secret("DB_PORT", &secret_store),
        db_url: get_secret("DB_URL", &secret_store),

    };

    let router = Router::new().route("/", get(handler)).with_state(secrets);

    Ok(router.into())
}

async fn handler(State(state): State<DatabaseSecrets>) -> String {
    format!(
        "{} - {} - {} - {} - {}",
        state.db_username, state.db_password, state.db_name, state.db_port, state.db_url
    )
}


fn get_secret (key: &str, secret_store: &SecretStore) -> String {
    match secret_store.get(key) {
        Some(secret) => {
            secret
        }
        None => {
            eprintln!("secret {key} was not found");
            "".to_string() // todo: propagate the error up to the caller (with anyhow), handle it there.
        }
    }
}