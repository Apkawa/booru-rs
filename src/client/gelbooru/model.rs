//! Models for Gelbooru
use core::fmt;

use serde::{Deserialize, Serialize};

/// Individual post from [`GelbooruResponse`]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GelbooruPost {
    /// The ID of the post
    pub id: u32,
    /// Datestamp of the post's creating date
    pub created_at: String,
    /// Post's score
    pub score: u32,
    /// Post's image width
    pub width: u32,
    /// Post's image height
    pub height: u32,
    /// Post's image md5
    pub md5: String,
    /// Post's image file url
    pub file_url: String,
    pub preview_url: String,

    pub sample_url: String,
    pub sample: bool,
    pub sample_height: u32,
    pub sample_width: u32,
    /// Post's tags
    pub tags: String,
    /// Post's image name (with extension)
    pub image: String,
    /// Post's image source
    pub source: String,
    /// Post's rating
    pub rating: GelbooruRating,


    pub directory: String,
    pub change: u32,
    pub owner: String,
    pub creator_id: u32,
    pub parent_id: u32,
    pub preview_height: u32,
    pub preview_width: u32,
    pub title: String,
    pub has_notes: String,
    pub has_comments: String,
    pub status: String,
    pub post_locked: u32,
    pub has_children: String,
}



/// Gelbooru's API response with a list a posts
#[derive(Serialize, Deserialize, Debug)]
pub struct GelbooruResponse {
    #[serde(rename = "post", default = "Vec::new")]
    pub posts: Vec<GelbooruPost>,
}

impl From<GelbooruResponse> for GelbooruPost {
    fn from(value: GelbooruResponse) -> Self {
        value.posts[0].to_owned()
    }
}

impl From<GelbooruResponse> for Vec<GelbooruPost> {
    fn from(value: GelbooruResponse) -> Self {
        value.posts
    }
}

/// Post's rating. Check the [Gelbooru's ratings wiki](https://gelbooru.com/index.php?page=help&topic=rating)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum GelbooruRating {
    Explicit,
    Questionable,
    Safe,
    Sensitive,
    General,
}

impl From<GelbooruRating> for String {
    fn from(rating: GelbooruRating) -> String {
        rating.to_string()
    }
}

impl fmt::Display for GelbooruRating {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lovercase_tag = format!("{:?}", self).to_lowercase();
        write!(f, "{lovercase_tag}")
    }
}

#[derive(Debug, Clone)]
pub enum GelbooruSort {
    Id,
    Score,
    Rating,
    User,
    Height,
    Width,
    Source,
    Updated,
    Random,
}

impl fmt::Display for GelbooruSort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lovercase_tag = format!("{:?}", self).to_lowercase();
        write!(f, "{lovercase_tag}")
    }
}
