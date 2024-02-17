use askama::Template;
use axum::response::{Html, IntoResponse, Response};

use http::StatusCode;

pub fn template_into_response<T: Template>(t: &T) -> Response {
    match t.render() {
        Ok(body) => {
            let headers = [(
                http::header::CONTENT_TYPE,
                http::HeaderValue::from_static(T::MIME_TYPE),
            )];

            (headers, body).into_response()
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub struct HtmlTemplate<T>(pub T);

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
