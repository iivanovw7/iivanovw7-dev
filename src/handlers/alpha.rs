use axum::{extract::State, response::IntoResponse};

use crate::{
    types::config::{AppState, BreadcrumbsConfig},
    utils::{breadcrumbs::generate_breadcrumbs, context::create_context},
};

use super::error::template_error;

pub async fn get(state: State<AppState>) -> impl IntoResponse {
    let tera = &state.tera;
    let mut context = create_context(&state);

    let breadcrumbs_config = BreadcrumbsConfig {
        path: "/samples/alpha".to_string(),
        leaf: None,
    };

    context.insert("breadcrumbs", &generate_breadcrumbs(breadcrumbs_config));

    match tera.render("pages/alpha/alpha.tera", &context) {
        Ok(body) => axum::response::Html(body).into_response(),
        Err(error) => template_error(state, error).await.into_response(),
    }
}
