use anyhow::Result;
use dotenv::dotenv;
use lazy_static::lazy_static;
use std::fs;
use struct_iterable::Iterable;
use tera::Tera;

use crate::types::config::{Config, Env, RawConfig, SocialConfig, SocialConfigItem};

lazy_static! {
    pub static ref TERA: Tera = {
        match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                panic!("Failed to create Tera instance: {}", e);
            }
        }
    };
}

lazy_static! {
    pub static ref ENV: Env = get_env().expect("Failed to load environment variables");
    pub static ref CONFIG: Config = get_config().expect("Failed to load configuration");
}

fn get_social_links(social: &SocialConfig) -> Vec<SocialConfigItem> {
    let mut links: Vec<SocialConfigItem> = Vec::new();

    for (key, value) in social.clone().iter() {
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

fn get_env() -> Result<Env> {
    dotenv().ok();
    Ok(envy::from_env::<Env>()?)
}

fn get_config() -> Result<Config> {
    let file = fs::read_to_string("config.toml").expect("Unable to read config.toml");
    let raw_config: RawConfig = toml::from_str(&file).expect("Unable to parse config.toml");
    let social_links = get_social_links(&raw_config.social);

    let mut config = Config {
        main: raw_config.main.clone(),
        jobs: raw_config.jobs.clone(),
        social: raw_config.social.clone(),
        social_links,
    };

    config.jobs.sort_by_key(|a| a.index);

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_env() {
        let env = get_env().expect("Failed to load environment variables");
        assert_ne!(env.server, "".to_string());
    }

    #[test]
    fn it_gets_env_from_the_lazy_static() {
        let env = &ENV;
        assert_ne!(env.server, "".to_string());
    }

    #[test]
    fn it_gets_social_links_directly() {
        let social_config = SocialConfig {
            email: "test@example.com".to_string(),
            github: "https://github.com/test".to_string(),
            linkedin: "https://linkedin.com/in/test".to_string(),
            telegram: "https://t.me/test".to_string(),
            twitter: "https://twitter.com/test".to_string(),
        };

        let links = get_social_links(&social_config);

        assert_eq!(links.len(), 5);
        assert_eq!(links[0].title, "email");
        assert_eq!(links[0].link, "test@example.com");
        assert_eq!(links[1].title, "github");
        assert_eq!(links[1].link, "https://github.com/test");
        assert_eq!(links[2].title, "linkedin");
        assert_eq!(links[2].link, "https://linkedin.com/in/test");
        assert_eq!(links[3].title, "telegram");
        assert_eq!(links[3].link, "https://t.me/test");
        assert_eq!(links[4].title, "twitter");
        assert_eq!(links[4].link, "https://twitter.com/test");
    }
}
