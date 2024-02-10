use dotenv::dotenv;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::fs;

#[derive(Clone, Deserialize, Debug)]
pub struct Env {
    pub server: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub main: MainConfig,
    pub social: SocialConfig,
    pub cards: CardsConfig,
}

#[derive(Clone, Deserialize, Debug)]
pub struct MainConfig {
    pub author: String,
    pub title: String,
    pub description: String,
    pub project: String,
    pub resume: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct SocialConfig {
    pub email: String,
    pub github: String,
    pub linkedin: String,
    pub telegram: String,
    pub twitter: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct CardsConfig {
    pub skills: CardConfig,
    pub contacts: CardConfig,
}

#[derive(Clone, Deserialize, Debug)]
pub struct CardConfig {
    pub title: String,
    pub description: String,
}

lazy_static! {
    pub static ref ENV: Env = get_env();
    pub static ref CONFIG: Config = get_config();
}

fn get_env() -> Env {
    dotenv().ok();

    match envy::from_env::<Env>() {
        Ok(env) => env,
        Err(error) => panic!("Env configuration Error: {:#?}", error),
    }
}

fn get_config() -> Config {
    let file = fs::read_to_string("config.toml").expect("Unable to read config.toml");
    let config: Config = toml::from_str(&file).expect("Unable to parse config.toml");

    return config.clone();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_env() {
        let env = get_env();
        assert_ne!(env.server, "".to_string());
    }

    #[test]
    fn it_gets_env_from_the_lazy_static() {
        let env = &ENV;
        assert_ne!(env.server, "".to_string());
    }
}
