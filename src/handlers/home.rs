use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

use crate::config::{CardsConfig, MainConfig, SocialConfig, CONFIG};

pub async fn get() -> impl IntoResponse {
    let template = HomeTemplate {
        main: CONFIG.main.clone(),
        social: CONFIG.social.clone(),
        cards: CONFIG.cards.clone(),
    };

    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "./pages/home.html")]
struct HomeTemplate {
    pub main: MainConfig,
    pub social: SocialConfig,
    pub cards: CardsConfig,
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
