use std::sync::Arc;
use dio_corebank::{
    config::{config_loader, config_model::DotEnvyConfig},
    infrastructure::{axum_http::http_servers::start, postgres::postgres_connector},
};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    println!("Welcome to Dio Banking na");
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let dotenvy_env: DotEnvyConfig = match config_loader::load() {
        Ok(env) => env,
        Err(e) => {
            error!("failed to load ENV: {}", e);
            std::process::exit(1);
        }
    };

    info!("ENV has been loaded : ");

    let postgres_pool = match postgres_connector::establish_connection(&dotenvy_env.database.url) {
        Ok(pool) => pool,
        Err(e) => {
            error!("failed to connect postgres: {}", e);
            std::process::exit(1);
        }
    };
    info!("postgres connection has been established");

    start(Arc::new(dotenvy_env), Arc::new(postgres_pool))
        .await
        .expect("Failed to start server");
}
