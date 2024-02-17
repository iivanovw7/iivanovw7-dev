use crate::{
    config::{MainConfig, SocialConfig, CONFIG},
    utils::HtmlTemplate,
};
use askama::Template;
use axum::response::IntoResponse;

pub async fn get() -> impl IntoResponse {
    let template = AlphaTemplate {
        main: CONFIG.main.clone(),
        social: CONFIG.social.clone(),
    };

    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "./pages/alpha.html")]
struct AlphaTemplate {
    pub main: MainConfig,
    pub social: SocialConfig,
}
