use axum::{extract::State, response::IntoResponse};
use tera::Context;

use crate::{types::config::AppState, utils::css::load_css_assets_manifest};

use super::error::template_error;

pub async fn get(state: State<AppState>) -> impl IntoResponse {
    let config = &state.config;
    let tera = &state.tera;
    let mut context = Context::new();

    context.insert("main", &config.main);
    context.insert("social", &config.social);
    context.insert("jobs", &config.jobs);
    context.insert("social_links", &config.social_links);
    context.insert("css_file", &load_css_assets_manifest());

    match tera.render("pages/home/home.html", &context) {
        Ok(body) => axum::response::Html(body).into_response(),
        Err(error) => template_error(state, error).await.into_response(),
    }
}
