use axum::{body::Body, extract::State, response::Response};
use http::StatusCode;
use serde_json::json;
use tera::{Context, Error};

use crate::{types::config::AppState, utils::css::load_css_assets_manifest};

pub async fn not_found_error(state: State<AppState>) -> Response {
    tracing::error!("Not found");

    render_error_page(state, StatusCode::NOT_FOUND, "Not found").await
}

pub async fn template_error(state: State<AppState>, error: Error) -> Response {
    tracing::error!("Template error: {}", error);

    render_error_page(
        state,
        StatusCode::INTERNAL_SERVER_ERROR,
        "Template Rendering Error",
    )
    .await
}

pub async fn internal_error(state: State<AppState>, error: anyhow::Error) -> Response {
    tracing::error!("Internal error: {}", error);

    render_error_page(
        state,
        StatusCode::INTERNAL_SERVER_ERROR,
        "Internal Server Error",
    )
    .await
}

async fn render_error_page(
    state: State<AppState>,
    status_code: StatusCode,
    message: &str,
) -> Response {
    let config = &state.config;
    let tera = &state.tera;
    let mut context = Context::new();

    let error = json!({
        "code": status_code.as_u16().to_string(),
        "message": message,
    });

    context.insert("main", &config.main);
    context.insert("social", &config.social);
    context.insert("css_file", &load_css_assets_manifest());
    context.insert("error", &error);

    match tera.render("pages/error/error.html", &context) {
        Ok(body) => Response::builder()
            .status(status_code)
            .header(axum::http::header::CONTENT_TYPE, mime::TEXT_HTML.as_ref())
            .body(Body::from(body))
            .unwrap(),
        Err(error) => {
            tracing::error!("Error rendering error page: {}", error);

            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(format!(
                    "Failed to render error page: {}",
                    error
                )))
                .unwrap()
        }
    }
}
