use askama::Template;
use axum::{extract::Multipart, response::Response};
use std::ffi::OsStr;
use std::path::Path;

use crate::utils::template_into_response;

#[derive(Template)]
#[template(path = "./components/alpha/alpha-report.html")]
struct AlphaReportTemplate {}

#[derive(Template)]
#[template(path = "./components/alpha/alpha-error.html")]
struct AlphaErrorTemplate {}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

pub async fn post(mut files: Multipart) -> Response {
    let template = AlphaReportTemplate {};
    let error = AlphaErrorTemplate {};

    while let Some(mut field) = files.next_field().await.unwrap() {
        let filename = if let Some(filename) = field.file_name() {
            filename.to_string()
        } else {
            continue;
        };

        match get_extension_from_filename(filename.as_str()) {
            Some("xml") => {
                let name = field.name().unwrap().to_string();
                let data = field.bytes().await.unwrap();

                println!("{}", name);

                return template_into_response(&template);
            }
            Some(&_) => continue,
            None => continue,
        }
    }

    return template_into_response(&error);
}
