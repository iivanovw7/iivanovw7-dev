use crate::config::{CONFIG, ENV, TERA};
use crate::handlers;
use crate::types::config::AppState;
use axum::{routing::get, Router};
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub async fn server() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    unsafe { std::env::set_var("RUST_LOG", "debug") };

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "iivanovw7-dev=debug".into()))
        .with(fmt::layer())
        .init();

    tracing::info!("initializing config");

    let config = Arc::new(CONFIG.clone());
    let tera = Arc::new(TERA.clone());

    tracing::info!("initializing state");

    let app_state = AppState {
        config: config.clone(),
        tera: tera.clone(),
    };

    tracing::info!("initializing assets");

    let current_dir = std::env::current_dir().unwrap();
    let current_dir_path = current_dir.to_str().unwrap();

    let assets_serve = ServeDir::new(format!("{}/assets", current_dir_path));
    let gsap_serve = ServeDir::new(format!("{}/node_modules/gsap/dist", current_dir_path));
    let htmx_serve = ServeDir::new(format!("{}/node_modules/htmx.org/dist", current_dir_path));
    let alpine_serve = ServeDir::new(format!("{}/node_modules/alpinejs/dist", current_dir_path));
    let morph_serve = ServeDir::new(format!(
        "{}/node_modules/@alpinejs/morph/dist",
        current_dir_path
    ));

    tracing::info!("initializing router");

    let router = Router::new()
        .route("/", get(handlers::home::get))
        .route("/samples/alpha", get(handlers::alpha::get))
        .nest_service("/assets", assets_serve.clone())
        .nest_service("/samples/assets", assets_serve.clone())
        .nest_service("/gsap/dist", gsap_serve)
        .nest_service("/htmx.org/dist", htmx_serve)
        .nest_service("/alpinejs/dist", alpine_serve)
        .nest_service("/@alpinejs/morph/dist", morph_serve)
        .fallback(get(handlers::error::not_found_error))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&ENV.server).await.unwrap();

    tracing::debug!("Router initialized, now listening on port {}", &ENV.server);

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
