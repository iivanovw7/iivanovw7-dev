use axum::{extract::State, response::IntoResponse};

use crate::{types::config::AppState, utils::context::create_context};

use super::error::template_error;

pub async fn get(state: State<AppState>) -> impl IntoResponse {
    let tera = &state.tera;

    let context = create_context(&state);

    match tera.render("pages/home/home.tera", &context) {
        Ok(body) => axum::response::Html(body).into_response(),
        Err(error) => template_error(state, error).await.into_response(),
    }
}
