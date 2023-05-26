use anyhow::anyhow;
use axum::{extract::State, routing::get, Router};
use shuttle_secrets::SecretStore;

#[shuttle_runtime::main]
async fn axum(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    let secret = if let Some(secret) = secret_store.get("DB_URL") {
        secret
    } else {
        return Err(anyhow!("secret was not found").into());
    };
    let state = DatabaseSecrets {
        db_username: secret_store.get("DB_USERNAME").unwrap(),
        db_password: secret_store.get("DB_PASSWORD").unwrap(),
        db_name: secret_store.get("DB_NAME").unwrap(),
        db_port: secret_store.get("DB_PORT").unwrap(),
        db_url: secret_store.get("DB_URL").unwrap(),

    };

    let router = Router::new().route("/", get(handler)).with_state(state);

    Ok(router.into())
}

async fn handler(State(state): State<DatabaseSecrets>) -> String {
    format!(
        "{} - {} - {} - {} - {}",
        state.db_username, state.db_password, state.db_name, state.db_port, state.db_url
    )
}

#[derive(Clone)]
struct DatabaseSecrets {
    db_username: String,
    db_password: String,
    db_name: String,
    db_port: String,
    db_url: String,
}
