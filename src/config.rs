use anyhow::Result;
use dotenv::dotenv;
use lazy_static::lazy_static;
use std::fs;
use struct_iterable::Iterable;
use tera::Tera;

use crate::types::config::{Config, Env, RawConfig, SocialConfig, SocialConfigItem};

lazy_static! {
    pub static ref TERA: Tera = {
        let mut tera = Tera::new();
        if let Err(e) = tera.load_from_glob("templates/**/*") {
            panic!("Failed to load templates: {}", e);
        }
        tera
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

    #[test]
    fn it_initializes_tera() {
        let _ = &*TERA;
    }

    #[test]
    fn it_renders_all_pages() {
        use crate::types::config::{AppState, BreadcrumbsConfig};
        use crate::types::posts::PostType;
        use crate::utils::breadcrumbs::generate_breadcrumbs;
        use crate::utils::context::create_context;
        use crate::utils::posts::collect_posts;
        use axum::extract::State;
        use serde_json::json;
        use std::sync::Arc;

        let state = State(AppState {
            config: Arc::new(CONFIG.clone()),
            tera: Arc::new(TERA.clone()),
        });

        // 1. Home page
        let ctx = create_context(&state);
        let home_rendered = TERA.render("pages/home/home.html", &ctx);
        assert!(
            home_rendered.is_ok(),
            "Failed to render home: {:?}",
            home_rendered.err()
        );

        // 2. Alpha page
        let mut alpha_ctx = create_context(&state);
        let breadcrumbs_config = BreadcrumbsConfig {
            path: "/samples/alpha".to_string(),
            leaf: None,
        };
        alpha_ctx.insert("breadcrumbs", &generate_breadcrumbs(breadcrumbs_config));
        let alpha_rendered = TERA.render("pages/alpha/alpha.html", &alpha_ctx);
        assert!(
            alpha_rendered.is_ok(),
            "Failed to render alpha: {:?}",
            alpha_rendered.err()
        );

        // 3. Posts page
        let (all_posts, all_tags) = collect_posts(vec![PostType::Project, PostType::Note]);
        let mut posts_ctx = create_context(&state);
        let breadcrumbs_config = BreadcrumbsConfig {
            path: "/posts".to_string(),
            leaf: None,
        };
        let current_tag: Option<String> = None;
        posts_ctx.insert("posts", &all_posts);
        posts_ctx.insert("tags", &all_tags);
        posts_ctx.insert("current_tag", &current_tag);
        posts_ctx.insert("breadcrumbs", &generate_breadcrumbs(breadcrumbs_config));
        let posts_rendered = TERA.render("pages/posts/posts.html", &posts_ctx);
        assert!(
            posts_rendered.is_ok(),
            "Failed to render posts: {:?}",
            posts_rendered.err()
        );

        // 4. Post page
        if let Some(post) = all_posts.first() {
            let mut post_ctx = create_context(&state);
            let breadcrumbs_config = BreadcrumbsConfig {
                path: format!("/posts/{}", post.metadata.file_name),
                leaf: Some(post.metadata.title.clone()),
            };
            post_ctx.insert("posts", &all_posts);
            post_ctx.insert("tags", &all_tags);
            post_ctx.insert("post", post);
            post_ctx.insert("breadcrumbs", &generate_breadcrumbs(breadcrumbs_config));
            let post_rendered = TERA.render("pages/post/post.html", &post_ctx);
            assert!(
                post_rendered.is_ok(),
                "Failed to render post: {:?}",
                post_rendered.err()
            );
        }

        // 5. Error page
        let mut error_ctx = create_context(&state);
        let error = json!({
            "code": "404",
            "message": "Not found",
        });
        error_ctx.insert("error", &error);
        let error_rendered = TERA.render("pages/error/error.html", &error_ctx);
        assert!(
            error_rendered.is_ok(),
            "Failed to render error: {:?}",
            error_rendered.err()
        );
    }
}
