use crate::config::ENV;
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

    let assets = std::env::current_dir().unwrap();
    let assets_path = assets.to_str().unwrap();

    let assets_serve = ServeDir::new(format!("{}/assets", assets_path));
    let gsap_serve = ServeDir::new(format!("{}/node_modules/gsap/dist", assets_path));
    let htmx_serve = ServeDir::new(format!("{}/node_modules/htmx.org/dist", assets_path));
    let alpine_serve = ServeDir::new(format!("{}/node_modules/alpinejs/dist", assets_path));
    let morph_serve = ServeDir::new(format!("{}/node_modules/@alpinejs/morph/dist", assets_path));

    let router = Router::new()
        .route("/", get(handlers::home::get))
        .route("/samples/alpha", get(handlers::alpha::get))
        .nest_service("/assets", assets_serve.clone())
        .nest_service("/samples/assets", assets_serve.clone())
        .nest_service("/gsap/dist", gsap_serve)
        .nest_service("/htmx.org/dist", htmx_serve)
        .nest_service("/alpinejs/dist", alpine_serve)
        .nest_service("/@alpinejs/morph/dist", morph_serve)
        .fallback(get(handlers::home::get));

    let listener = tokio::net::TcpListener::bind(&ENV.server).await.unwrap();

    tracing::debug!("Router initialized, now listening on port {}", &ENV.server);

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
