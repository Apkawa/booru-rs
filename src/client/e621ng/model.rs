//! Models for Danbooru
use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct E621ngListResponse {
    posts: Vec<E621ngPost>
}

impl From<E621ngListResponse> for Vec<E621ngPost> {
    fn from(value: E621ngListResponse) -> Self {
        value.posts
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct E621ngDetailResponse {
    post: E621ngPost
}

impl From<E621ngDetailResponse> for E621ngPost {
    fn from(value: E621ngDetailResponse) -> Self {
        value.post
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct E621ngPost {
    pub id: u32,
    pub created_at: String,
    pub updated_at: String,
    pub file: File,
    pub preview: Preview,
    pub sample: Sample,
    pub score: Score,
    pub tags: Tags,
    // pub locked_tags: Vec<_>,
    pub change_seq: u32,
    pub flags: Flags,
    pub rating: String,
    pub fav_count: u32,
    pub sources: Vec<String>,
    // pub pools: Vec<_>,
    // pub relationships: Relationships,
    // pub approver_id: Option<_>,
    pub uploader_id: u32,
    pub description: String,
    pub comment_count: u32,
    pub is_favorited: bool,
    pub has_notes: bool,
    // pub duration: Option<_>,
}


// #[derive(Serialize, Deserialize)]
// struct Relationships {
//     pub parent_id: Option<_>,
//     pub has_children: bool,
//     pub has_active_children: bool,
//     pub children: Vec<_>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Flags {
    pub pending: bool,
    pub flagged: bool,
    pub note_locked: bool,
    pub status_locked: bool,
    pub rating_locked: bool,
    pub comment_disabled: bool,
    pub deleted: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tags {
    pub general: Vec<String>,
    pub species: Vec<String>,
    pub character: Vec<String>,
    pub copyright: Vec<String>,
    pub artist: Vec<String>,
    pub invalid: Vec<String>,
    pub lore: Vec<String>,
    pub meta: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Score {
    pub up: u32,
    pub down: i32,
    pub total: i32,
}

// #[derive(Serialize, Deserialize)]
// struct Alternates {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sample {
    pub has: bool,
    pub height: u32,
    pub width: u32,
    pub url: Option<String>,
    // pub alternates: Alternates,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Preview {
    pub width: u32,
    pub height: u32,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct File {
    pub width: u32,
    pub height: u32,
    pub ext: String,
    pub size: usize,
    pub md5: String,
    pub url: Option<String>,
}




/// Post's rating. Check the [Ratings wiki](https://e621.net/help/ratings)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum E621ngRating {
    #[serde(rename = "e")]
    Explicit,
    #[serde(rename = "q")]
    Questionable,
    #[serde(rename = "s")]
    Safe,
}

impl From<E621ngRating> for String {
    fn from(rating: E621ngRating) -> String {
        rating.to_string()
    }
}

impl fmt::Display for E621ngRating {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lowercase_tag = format!("{:?}", self).to_lowercase();
        write!(f, "{lowercase_tag}")
    }
}
// TODO fill from https://e621.net/wiki_pages/9169
#[derive(Debug, Clone)]
pub enum E621ngSort {
    Id,
    Score,
    Rating,
    Height,
    Width,
    Change,
    Random,
}

impl fmt::Display for E621ngSort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lowercase_tag = format!("{:?}", self).to_lowercase();
        write!(f, "{lowercase_tag}")
    }
}
