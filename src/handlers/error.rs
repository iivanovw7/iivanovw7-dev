use axum::{
    body::Body,
    extract::State,
    response::{IntoResponse, Response},
};
use http::StatusCode;
use serde_json::json;
use tera::Error;

use crate::{types::config::AppState, utils::context::create_context};

pub async fn post_not_found(state: State<AppState>, post_name: &String) -> impl IntoResponse {
    let message = format!("Post not found: {}", post_name);

    tracing::error!(message);

    render_error_page(state, StatusCode::NOT_FOUND, &message)
        .await
        .into_response()
}

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
    let tera = &state.tera;

    let mut context = create_context(&state);

    let error = json!({
        "code": status_code.as_u16().to_string(),
        "message": message,
    });

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
