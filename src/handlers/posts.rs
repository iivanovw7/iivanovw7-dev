use axum::{
    extract::{Query, State},
    response::IntoResponse,
};

use crate::{
    types::{
        config::{AppState, BreadcrumbsConfig},
        posts::{PostType, PostsQuery},
    },
    utils::{breadcrumbs::generate_breadcrumbs, context::create_context, posts::collect_posts},
};

use super::error::template_error;

pub async fn get(
    Query(posts_query): Query<PostsQuery>,
    state: State<AppState>,
) -> impl IntoResponse {
    let tera = &state.tera;
    let current_tag = posts_query.tag;

    let breadcrumbs_config = BreadcrumbsConfig {
        path: "/posts".to_string(),
        leaf: None,
    };

    let mut context = create_context(&state);

    let (all_posts, all_tags) = collect_posts(vec![PostType::Project, PostType::Note]);

    let mut posts = all_posts.clone();

    if let Some(tag) = &current_tag {
        posts.retain(|post| post.metadata.tags.contains(tag));
    }

    context.insert("posts", &posts);
    context.insert("tags", &all_tags.clone());
    context.insert("current_tag", &current_tag);
    context.insert("breadcrumbs", &generate_breadcrumbs(breadcrumbs_config));

    match tera.render("pages/posts/posts.tera", &context) {
        Ok(body) => axum::response::Html(body).into_response(),
        Err(error) => template_error(state, error).await.into_response(),
    }
}
