use axum::extract::State;
use tera::Context;

use crate::types::config::AppState;

use std::fs;

use crate::types::config::CssManifest;

fn load_css_assets_manifest() -> String {
    let manifest_content = fs::read_to_string("assets/css/manifest.json").unwrap();
    let manifest: CssManifest = serde_json::from_str(&manifest_content).unwrap();

    manifest.entries["main.scss"].clone()
}

pub fn create_context(state: &State<AppState>) -> Context {
    let config = &state.config;

    let mut context = Context::new();

    context.insert("main", &config.main);
    context.insert("social", &config.social);
    context.insert("social_links", &config.social_links);
    context.insert("jobs", &config.jobs);
    context.insert("css_file", &load_css_assets_manifest());

    context
}
