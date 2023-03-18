//! Models for Zerochan
/// https://www.zerochan.net/api
use core::fmt;

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZerochanPost {
    pub id: u32,
    pub small: String,
    pub medium: String,
    pub large: String,
    pub full: String,
    pub width: u32,
    pub height: u32,
    pub size: usize,
    pub hash: Option<String>,
    pub source: Option<String>,
    pub primary: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ZerochanListResponse {
    pub items: Vec<ZerochanListItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ZerochanListItem {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub thumbnail: String,
    pub source: Option<String>,
    pub tag: String,
    pub tags: Vec<String>,
}

impl From<ZerochanListResponse> for Vec<ZerochanPost> {
    fn from(value: ZerochanListResponse) -> Self {
        value.items.into_iter().map(|v| v.into()).collect()
    }
}

impl From<ZerochanListItem> for ZerochanPost {
    fn from(value: ZerochanListItem) -> Self {
        let ZerochanListItem {
            id,
            width, height,
            thumbnail,
            source,
            tag, tags
        } = value;
        let tag_img = tag.replace(' ', ".").replace('&', ".");
        let tag_img: String = form_urlencoded::byte_serialize(tag_img.as_bytes()).collect();
        ZerochanPost {
            id,
            medium: thumbnail.clone(),
            small: thumbnail.replace("net/240/", "net/75/"),
            large: format!("https://s1.zerochan.net/{tag_img}.600.{id}.jpg"),
            full: format!("https://static.zerochan.net/{tag_img}.full.{id}.jpg"),
            width,
            height,
            size: 0,
            hash: None,
            source,
            primary: tag,
            tags,
        }
    }
}


/// Post's rating.
// TODO find tag
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ZerochanRating {
    Safe,
    Explicit,
    Questionable,
    Suggestive,
    SemiDashGrimdark,
    Grimdark,
    Grotesque,
}

impl From<ZerochanRating> for String {
    fn from(rating: ZerochanRating) -> String {
        rating.to_string()
    }
}

impl fmt::Display for ZerochanRating {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lowercase_tag = if let ZerochanRating::SemiDashGrimdark = self {
            "semi-dash-grimdark".to_string()
        } else {
            format!("{:?}", self).to_lowercase()
        };
        write!(f, "{lowercase_tag}")
    }
}

#[derive(Debug, Clone)]
pub enum ZerochanSort {
    Id,
    Fav,
}

impl fmt::Display for ZerochanSort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lowercase_tag = format!("{:?}", self).to_lowercase();
        write!(f, "{lowercase_tag}")
    }
}