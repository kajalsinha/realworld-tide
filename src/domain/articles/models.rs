use crate::domain::{Profile, ProfileView};
use chrono::{DateTime, Utc};
use derive_more::Constructor;
use itertools::Itertools;
use uuid::Uuid;

#[derive(Clone, Constructor, Debug, PartialEq)]
pub struct ArticleContent {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
}

impl ArticleContent {
    /// Convert a title into a url-safe slug
    pub fn slug(&self) -> String {
        self.title
            .to_ascii_lowercase()
            .split_ascii_whitespace()
            .join("-")
    }
}

#[derive(Clone, Constructor, Debug, PartialEq)]
pub struct Article {
    pub content: ArticleContent,
    pub slug: String,
    pub author: Profile,
    pub metadata: ArticleMetadata,
    pub favorites_count: u64,
}

#[derive(Clone, Constructor, Debug, PartialEq)]
pub struct ArticleMetadata {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Constructor, Debug, PartialEq)]
pub struct ArticleView {
    pub content: ArticleContent,
    pub slug: String,
    pub author: ProfileView,
    pub metadata: ArticleMetadata,
    pub favorited: bool,
    pub favorites_count: u64,
    // The user owning this view of an article
    pub viewer: Uuid,
}
