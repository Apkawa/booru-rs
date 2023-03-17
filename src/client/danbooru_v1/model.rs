//! Models for DanbooruV1
use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DanbooruPostV1 {
    pub id: u32,
    pub status: String,
    pub creator_id: u32,
    pub author: String,
    pub score: i32,
    pub has_comments: bool,
    pub has_children: bool,
    pub parent_id: Option<u32>,
    pub md5: String,
    pub tags: String,
    pub change: u32,
    pub has_notes: bool,
    pub rating: String,

    pub sample_width: u32,
    pub sample_height: u32,
    pub sample_url: String,

    pub preview_width: u32,
    pub preview_height: u32,
    pub preview_url: String,

    pub source: String,
    pub height: u32,
    pub width: u32,
    pub file_url: String,
    pub file_size: usize,
    pub created_at: CreatedAt,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatedAt {
    pub n: i64,
    pub s: i64,
    pub json_class: String,
}


impl From<Vec<DanbooruPostV1>> for DanbooruPostV1 {
    fn from(value: Vec<DanbooruPostV1>) -> Self {
        value[0].to_owned()
    }
}

/// Post's rating. Check the [Danbooru's ratings wiki](https://danbooru.donmai.us/wiki_pages/howto:rate)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DanbooruRatingV1 {
    #[serde(rename = "e")]
    Explicit,
    #[serde(rename = "q")]
    Questionable,
    #[serde(rename = "s")]
    Sensitive,
    #[serde(rename = "g")]
    General,
}

impl From<DanbooruRatingV1> for String {
    fn from(rating: DanbooruRatingV1) -> String {
        rating.to_string()
    }
}

impl fmt::Display for DanbooruRatingV1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lowercase_tag = format!("{:?}", self).to_lowercase();
        write!(f, "{lowercase_tag}")
    }
}

// http://behoimi.org/help/cheatsheet
#[derive(Debug, Clone)]
pub enum DanbooruSortV1 {
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

impl fmt::Display for DanbooruSortV1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lowercase_tag = format!("{:?}", self).to_lowercase();
        write!(f, "{lowercase_tag}")
    }
}
