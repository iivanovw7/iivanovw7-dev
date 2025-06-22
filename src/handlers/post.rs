use axum::{
    extract::{Path, State},
    response::IntoResponse,
};

use crate::{
    types::{
        config::{AppState, BreadcrumbsConfig},
        posts::{Post, PostType},
    },
    utils::{breadcrumbs::generate_breadcrumbs, context::create_context, posts::collect_posts},
};

use super::error::{post_not_found, template_error};

pub async fn get(Path(post_name): Path<String>, state: State<AppState>) -> impl IntoResponse {
    let tera = &state.tera;

    let mut context = create_context(&state);

    let (all_posts, all_tags) = collect_posts(vec![PostType::Project, PostType::Note]);

    context.insert("posts", &all_posts.clone());
    context.insert("tags", &all_tags.clone());

    let found_post = all_posts
        .iter()
        .find(|&p| p.metadata.file_name == post_name);

    if let Some(found_post) = found_post {
        let post: Post = found_post.clone();

        let breadcrumbs_config = BreadcrumbsConfig {
            path: format!("/posts/{}", post_name),
            leaf: Some(post.metadata.title.clone()),
        };

        context.insert("post", &post);
        context.insert("breadcrumbs", &generate_breadcrumbs(breadcrumbs_config));

        match tera.render("pages/post/post.tera", &context) {
            Ok(body) => axum::response::Html(body).into_response(),
            Err(error) => template_error(state, error).await.into_response(),
        }
    } else {
        post_not_found(state, &post_name).await.into_response()
    }
}
