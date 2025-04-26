use crate::types::posts::{Post, PostMetadata, PostType};
use chrono::NaiveDate;
use std::{
    collections::HashMap,
    fs::{self, DirEntry},
    path::Path,
};

use super::string::capitalize;

pub fn get_posts_file<P: AsRef<Path>>(path: P) -> Vec<DirEntry> {
    fs::read_dir(path)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|entry| {
            if let Ok(file_type) = entry.file_type() {
                file_type.is_file() && entry.path().extension() == Some("md".as_ref())
            } else {
                false
            }
        })
        .collect()
}

pub fn read_post_content(entry: DirEntry) -> Option<String> {
    fs::read_to_string(entry.path()).ok()
}

pub fn parse_post_content(content: &str) -> Option<Post> {
    use gray_matter::engine::YAML;
    use gray_matter::Matter;
    use pulldown_cmark::{html, Options, Parser};

    let mut options = Options::empty();

    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let matter = Matter::<YAML>::new();
    let post_data = matter
        .parse_with_struct::<PostMetadata>(content)
        .expect("Unable to parse md frontmatter");

    let metadata = post_data.data;
    let content = post_data.content;
    let parser = Parser::new_ext(&content, options);

    let mut html_output = String::new();

    html::push_html(&mut html_output, parser);

    let date = parse_date(&metadata.date).format("%B %e, %Y").to_string();

    Some(Post::new(metadata, html_output, date))
}

pub fn sort_posts(posts: &mut [Post]) {
    posts.sort_by(|a, b| {
        let a_date = parse_date(&a.metadata.date);
        let b_date = parse_date(&b.metadata.date);

        a_date.cmp(&b_date)
    });
}

pub fn parse_date(date: &str) -> NaiveDate {
    NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap()
}

pub fn process_posts<P: AsRef<Path>>(path: P) -> (Vec<Post>, Vec<String>) {
    let posts_text = get_posts_file(path);
    let mut posts = Vec::new();
    let mut tags = Vec::new();

    for entry in posts_text {
        if let Some(content) = read_post_content(entry) {
            if let Some(mut post) = parse_post_content(&content) {
                let mut post_tags = Vec::new();

                let parsed_post_date = parse_date(&post.metadata.date);

                for post_tag in post.metadata.tags.clone() {
                    post_tags.push(capitalize(post_tag.leak()))
                }

                post.metadata.date = parsed_post_date.format("%Y-%m-%d").to_string();

                posts.push(post);

                tags.append(&mut post_tags);
            }
        }
    }

    sort_posts(&mut posts);

    (posts, tags)
}

pub fn collect_posts() -> (HashMap<PostType, Vec<Post>>, Vec<String>) {
    let mut post_paths = HashMap::new();

    post_paths.insert(PostType::Note, "posts/notes");
    post_paths.insert(PostType::Project, "posts/projects");

    let mut all_posts = HashMap::new();
    let mut all_tags = Vec::new();

    for (post_type, path) in post_paths {
        let (posts, mut tags) = process_posts(path);

        all_posts.insert(post_type, posts);
        all_tags.append(&mut tags);
        all_tags.sort();
    }

    (all_posts, all_tags)
}
