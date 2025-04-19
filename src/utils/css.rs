use std::fs;

use crate::types::config::CssManifest;

pub fn load_css_assets_manifest() -> String {
    let manifest_content = fs::read_to_string("assets/css/manifest.json").unwrap();
    let manifest: CssManifest = serde_json::from_str(&manifest_content).unwrap();

    manifest.entries["main.scss"].clone()
}
