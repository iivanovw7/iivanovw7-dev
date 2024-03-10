use crate::{
    config::{MainConfig, CONFIG},
    utils::HtmlTemplate,
};
use askama::Template;
use axum::response::IntoResponse;

pub async fn get() -> impl IntoResponse {
    let template = AlphaTemplate {
        #[allow(dead_code)]
        main: CONFIG.main.clone(),
    };

    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "./pages/alpha.html")]
struct AlphaTemplate {
    pub main: MainConfig,
}
