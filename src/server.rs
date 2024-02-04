use crate::config::CONFIG;
use crate::handlers;
use axum::{routing::get, Router};
use tower_http::services::ServeDir;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub async fn server() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "iivanovw7-dev=debug".into()))
        .with(fmt::layer())
        .init();

    tracing::info!("initializing router and assets");

    let assets_path = std::env::current_dir().unwrap();
    let router = Router::new()
        .route("/", get(handlers::home::get))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        );

    let listener = tokio::net::TcpListener::bind(&CONFIG.server).await.unwrap();

    tracing::debug!(
        "Router initialized, now listening on port {}",
        &CONFIG.server
    );

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
