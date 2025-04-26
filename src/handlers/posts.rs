use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use itertools::Itertools;
use tera::Context;

use crate::{
    types::{
        config::AppState,
        posts::{Post, PostType, PostsQuery},
    },
    utils::{css::load_css_assets_manifest, posts::collect_posts},
};

use super::error::template_error;

pub async fn get(
    Query(posts_query): Query<PostsQuery>,
    state: State<AppState>,
) -> impl IntoResponse {
    let config = &state.config;
    let tera = &state.tera;
    let current_tag = posts_query.tag;

    let mut context = Context::new();

    let (all_posts, all_tags) = collect_posts();

    let mut posts: Vec<Post> = vec![];

    let mut notes = all_posts.get(&PostType::Note).unwrap().clone();
    let mut projects = all_posts.get(&PostType::Project).unwrap().clone();

    let tags: Vec<String> = all_tags.into_iter().unique().collect();

    posts.append(&mut notes);
    posts.append(&mut projects);

    if let Some(tag) = &current_tag {
        posts.retain(|post| post.metadata.tags.contains(tag));
    }

    context.insert("main", &config.main);
    context.insert("social", &config.social);
    context.insert("css_file", &load_css_assets_manifest());
    context.insert("posts", &posts);
    context.insert("tags", &tags);
    context.insert("current_tag", &current_tag);

    match tera.render("pages/posts/posts.html", &context) {
        Ok(body) => axum::response::Html(body).into_response(),
        Err(error) => template_error(state, error).await.into_response(),
    }
}
