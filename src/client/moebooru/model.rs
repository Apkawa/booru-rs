//! Models for Danbooru
use core::fmt;

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MoebooruPost {
    pub id: u32,
    pub tags: String,
    pub created_at: u32,
    pub creator_id: u32,
    pub author: String,
    pub change: u32,
    pub source: String,
    pub score: i32,
    pub md5: String,
    pub file_size: usize,
    pub file_url: String,
    pub is_shown_in_index: bool,
    pub preview_url: String,
    pub preview_width: usize,
    pub preview_height: usize,
    pub actual_preview_width: usize,
    pub actual_preview_height: usize,
    pub sample_url: String,
    pub sample_width: u32,
    pub sample_height: u32,
    pub sample_file_size: usize,
    pub jpeg_url: String,
    pub jpeg_width: u32,
    pub jpeg_height: u32,
    pub jpeg_file_size: usize,
    pub rating: String,
    pub has_children: bool,
    pub parent_id: Option<u32>,
    pub status: String,
    pub width: u32,
    pub height: u32,
    pub is_held: bool,
}

impl From<Vec<MoebooruPost>> for MoebooruPost {
    fn from(value: Vec<MoebooruPost>) -> Self {
        value[0].to_owned()
    }
}

/// Post's rating. Check the [Moebooru cheatcheet](https://konachan.com/help/cheatsheet)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MoebooruRating {
    Explicit,
    Questionable,
    Sensitive,
    General,
}

impl From<MoebooruRating> for String {
    fn from(rating: MoebooruRating) -> String {
        rating.to_string()
    }
}

impl fmt::Display for MoebooruRating {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lowercase_tag = format!("{:?}", self).to_lowercase();
        write!(f, "{lowercase_tag}")
    }
}

#[derive(Debug, Clone)]
pub enum MoebooruSort {
    Id,
    Score,
    MPixels,
    Landscape,
    Portrait,
    Vote,
    Random,
}

impl fmt::Display for MoebooruSort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lowercase_tag = format!("{:?}", self).to_lowercase();
        write!(f, "{lowercase_tag}")
    }
}
