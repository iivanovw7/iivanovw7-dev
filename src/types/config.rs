use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use struct_iterable::Iterable;
use tera::Tera;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CssManifest {
    #[serde(flatten)]
    pub entries: HashMap<String, String>,
}

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub tera: Arc<Tera>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Env {
    pub server: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RawConfig {
    pub main: MainConfig,
    pub social: SocialConfig,
    pub jobs: [Job; 4],
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub main: MainConfig,
    pub social: SocialConfig,
    pub jobs: [Job; 4],
    pub social_links: Vec<SocialConfigItem>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SocialConfigItem {
    pub title: String,
    pub link: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MainConfig {
    pub author: String,
    pub name: String,
    pub profession: String,
    pub project: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Iterable)]
pub struct SocialConfig {
    pub email: String,
    pub github: String,
    pub linkedin: String,
    pub telegram: String,
    pub twitter: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Job {
    pub index: i16,
    pub end: String,
    pub company: String,
    pub company_logo: String,
    pub location: String,
    pub position: String,
    pub start: String,
    pub subtitle: String,
}
