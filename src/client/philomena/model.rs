//! Models for Philomena
/// https://derpibooru.org/pages/api
use core::fmt;

use serde::{Deserialize, Serialize};
use crate::client::philomena::PhilomenaRating::Grimdark;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhilomenaPost {
    pub id: u32,
    pub name: String,
    pub description: String,

    pub width: u32,
    pub height: u32,
    pub source_url: Option<String>,
    pub view_url: String,
    pub size: usize,
    pub orig_sha512_hash: Option<String>,
    pub format: String,
    pub aspect_ratio: f32,
    pub mime_type: String,
    pub duration: f64,
    pub processed: bool,
    pub sha512_hash: String,
    pub animated: bool,
    pub thumbnails_generated: bool,
    pub representations: Representations,

    pub tags: Vec<String>,
    pub tag_count: u32,
    pub tag_ids: Vec<u32>,

    pub wilson_score: f64,
    pub score: i32,
    pub comment_count: u32,
    pub downvotes: u32,
    pub upvotes: u32,

    pub spoilered: bool,
    pub faves: u32,
    pub hidden_from_users: bool,

    pub first_seen_at: String,
    pub created_at: String,
    pub updated_at: String,

    pub uploader: Option<String>,
    pub uploader_id: Option<u32>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Representations {
    pub full: String,
    pub large: String,
    pub medium: String,
    pub small: String,
    pub tall: String,
    pub thumb: String,
    pub thumb_small: String,
    pub thumb_tiny: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PhilomenaDetailResponse {
    pub image: PhilomenaPost,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PhilomenaListResponse {
    pub images: Vec<PhilomenaPost>,
}

impl From<PhilomenaListResponse> for Vec<PhilomenaPost> {
    fn from(value: PhilomenaListResponse) -> Self {
        value.images
    }
}

impl From<PhilomenaDetailResponse> for PhilomenaPost {
    fn from(value: PhilomenaDetailResponse) -> Self {
        value.image
    }
}

/// Post's rating. Check the [Category tags](https://derpibooru.org/tags?tq=category%3Arating)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PhilomenaRating {
    Safe,
    Explicit,
    Questionable,
    Suggestive,
    SemiDashGrimdark,
    Grimdark,
    Grotesque,
}

impl From<PhilomenaRating> for String {
    fn from(rating: PhilomenaRating) -> String {
        rating.to_string()
    }
}

impl fmt::Display for PhilomenaRating {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lowercase_tag = if let PhilomenaRating::SemiDashGrimdark = self {
            "semi-dash-grimdark".to_string()
        } else {
            format!("{:?}", self).to_lowercase()
        };
        write!(f, "{lowercase_tag}")
    }
}

/// See [Philomena search syntax](https://derpibooru.org/pages/search_syntax)
#[derive(Debug, Clone)]
pub enum PhilomenaSort {
    Id,
    Score,
    Width,
    Height,
    Upvotes,
    Downvotes,
    // AspectRatio // TODO convert to aspect_ratio
    // CommentCount
    // CreatedAt
}

impl fmt::Display for PhilomenaSort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lowercase_tag = format!("{:?}", self).to_lowercase();
        write!(f, "{lowercase_tag}")
    }
}