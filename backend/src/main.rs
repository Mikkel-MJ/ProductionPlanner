use jsonwebtoken::jwk;
use sqlx::{Pool, Postgres};
use std::env;

use crate::middlewares::auth::AuthConfig;

mod handlers;
mod middlewares;
mod model;

pub mod app;
pub mod db;

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
}
#[tokio::main]
async fn main() {
    println!("Starting server..");
    // debug_set_env_var(); // only for testing

    let conn_string = env::var("DATABASE_URL").expect("did not find DATABASE_URL env var");
    println!("connstring: {}", conn_string);
    let db_pool = db::new_db_pool(5, &conn_string).await;
    db::migrate_db(&db_pool).await;
    println!("migration completed");

    let auth_config = AuthConfig {
        audience: env::var("AUDIENCE").expect("AUDIENCE needs to be set"),
        authority: env::var("AUTHORITY").expect("AUTHORITY needs to be set"),
        jwks: setup_jwks_secrets(env::var("JWKS_URI").expect("JWKS_URI needs to be set")).await,
    };

    let state = AppState { db: db_pool };

    let app = app::create_app(state, auth_config).await;

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn debug_set_env_var() {
    env::set_var(
        "DATABASE_URL",
        "postgres://postgres:passwordPG@localhost:5432/postgres",
    );
    env::set_var("AUTHORITY", "https://dev-o5vpiij8.eu.auth0.com/");
    env::set_var("AUDIENCE", "prod-plan-api");
    env::set_var(
        "JWKS_URI",
        "https://dev-o5vpiij8.eu.auth0.com/.well-known/jwks.json",
    )
}

async fn setup_jwks_secrets(jwk_url: String) -> jwk::JwkSet {
    let jwks_string = reqwest::get(jwk_url)
        .await
        .expect("failed to get jwks")
        .text()
        .await
        .expect("error reading jwks");

    let jwks: jwk::JwkSet = serde_json::from_str(&jwks_string).unwrap();
    jwks
}
