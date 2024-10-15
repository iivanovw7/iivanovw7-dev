use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use struct_iterable::Iterable;

use crate::config::{Job, MainConfig, SocialConfig, SocialConfigItem, CONFIG};

fn get_social_links() -> Vec<SocialConfigItem> {
    let mut links: Vec<SocialConfigItem> = Vec::new();

    for (key, value) in CONFIG.social.clone().iter() {
        links.push(SocialConfigItem {
            title: String::from(key),
            link: match &value.downcast_ref::<String>() {
                Some(as_string) => as_string.to_string(),
                None => "".to_string(),
            },
        })
    }

    links
}

pub async fn get() -> impl IntoResponse {
    let template = HomeTemplate {
        main: CONFIG.main.clone(),
        social: CONFIG.social.clone(),
        social_links: get_social_links(),
        jobs: CONFIG.jobs.clone(),
    };

    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "./pages/home.html")]
struct HomeTemplate {
    pub main: MainConfig,
    #[allow(dead_code)]
    pub social: SocialConfig,
    pub social_links: Vec<SocialConfigItem>,
    pub jobs: [Job; 4],
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
