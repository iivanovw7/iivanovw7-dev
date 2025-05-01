use serde::{Deserialize, Serialize};
use std::fmt::{Formatter, Result};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostsQuery {
    pub tag: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostMetadata {
    pub image: String,
    pub title: String,
    pub date: String,
    pub description: String,
    pub link: String,
    pub file_name: String,
    pub tags: Vec<String>,
}

impl PostMetadata {
    pub fn create_href(&self) -> String {
        self.title.replace(' ', "-").to_lowercase()
    }
}

pub type PostContent = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub metadata: PostMetadata,
    pub content: PostContent,
    pub date: String,
}

impl Post {
    pub fn new(metadata: PostMetadata, content: PostContent, date: String) -> Self {
        Self {
            metadata,
            content,
            date,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PostType {
    Note,
    Project,
}

impl std::fmt::Display for PostType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            PostType::Note => write!(f, "notes"),
            PostType::Project => write!(f, "projects"),
        }
    }
}
