//! Models for Zerochan
/// https://www.zerochan.net/api
use core::fmt;
use std::borrow::Cow;

use crate::client::generic::model::{BooruPostModelSetUrl, Image, ImageHash, Images};
use crate::client::generic::BooruPostModel;
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
    /// md5
    pub hash: Option<String>,
    pub source: Option<String>,
    pub primary: String,
    pub tags: Vec<String>,

    pub base_url: Option<String>,
}

impl BooruPostModel for ZerochanPost {
    fn id(&self) -> Cow<str> {
        self.id.to_string().into()
    }

    fn hash(&self) -> Option<ImageHash> {
        self.hash.as_ref().map(|s| ImageHash::MD5(s.as_str()))
    }

    fn images(&self) -> Images {
        Images {
            original: Image::new(self.full.as_str())
                .size(self.width, self.height)
                .filesize(self.size)
                .into(),
            sample: Image::new(self.large.as_str()).into(),
            preview: Image::new(self.medium.as_str()).into(),
        }
    }

    fn source_url(&self) -> Option<Cow<str>> {
        self.source.as_ref().map(|source| source.into())
    }

    fn tags(&self) -> Vec<Cow<str>> {
        self.tags.iter().map(Into::into).collect()
    }

    fn post_url(&self) -> Option<Cow<str>> {
        Some(format!("{}/{}", self.base_url.as_ref().unwrap(), self.id).into())
    }
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
            width,
            height,
            thumbnail,
            source,
            tag,
            tags,
        } = value;
        let tag_img = tag.replace([' ', '&'], ".");
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
            base_url: None,
        }
    }
}

impl BooruPostModelSetUrl for ZerochanPost {
    fn set_base_url<I: Into<String>>(mut self, url: I) -> Self
        where Self: Sized {
        self.base_url = Some(url.into());
        self
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
