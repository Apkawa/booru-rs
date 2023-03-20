//! Models for DanbooruV1
use core::fmt;
use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::client::generic::BooruPostModel;
use crate::client::generic::model::{BooruPostModelSetUrl, Image, ImageHash, Images};
use crate::utils::dt::timestamp_to_rfc_3339;

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

    pub height: u32,
    pub width: u32,
    pub file_url: String,
    pub file_size: usize,

    pub source: String,

    pub created_at: CreatedAt,

    pub base_url: Option<String>
}

impl BooruPostModel for DanbooruPostV1 {
    fn id(&self) -> Cow<str> {
        self.id.to_string().into()
    }

    fn hash(&self) -> Option<ImageHash> {
        Some(ImageHash::MD5(self.md5.as_str()))
    }

    fn images(&self) -> Images {
        Images {
            original: Image::new(self.file_url.as_str())
                .filesize(self.file_size)
                .size(self.width, self.height)
                .into(),
            sample: Image::new(self.sample_url.as_str())
                .size(self.sample_width, self.sample_height)
                .into(),
            preview: Image::new(self.preview_url.as_str())
                .size(self.preview_width, self.preview_height)
                .into(),
        }
    }

    fn source_url(&self) -> Option<Cow<str>> {
        if !self.source.is_empty() {
            Some(self.source.as_str().into())
        } else {
            None
        }
    }

    fn tags(&self) -> Vec<Cow<str>> {
        self.tags.split(' ').map(Into::into).collect()
    }

    fn created(&self) -> Option<Cow<str>> {
        Some(self.created_at.s.as_str().into())
    }

    fn post_url(&self) -> Option<Cow<str>> {
        Some(format!("{}/post/show/{}/", self.base_url.as_ref().unwrap(), self.id).into())
    }
}

impl BooruPostModelSetUrl for DanbooruPostV1 {
    fn set_base_url<I: Into<String>>(mut self, url: I) -> Self
        where Self: Sized {
        self.base_url = Some(url.into());
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatedAt {
    pub n: u64,
    #[serde(with = "timestamp_to_rfc_3339")]
    pub s: String,
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
