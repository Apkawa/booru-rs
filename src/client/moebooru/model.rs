//! Models for Danbooru
use core::fmt;
use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::client::generic::BooruPostModel;
use crate::client::generic::model::{Image, ImageHash, Images};
use crate::utils::dt::timestamp_to_rfc_3339;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MoebooruPost {
    pub id: u32,
    pub tags: String,
    #[serde(with = "timestamp_to_rfc_3339")]
    pub created_at: String,
    pub creator_id: u32,
    pub author: String,
    pub change: u32,
    pub source: String,
    // for compatibly myimouto
    // pub score: i32,
    pub md5: String,
    pub file_size: usize,
    pub file_url: String,
    pub is_shown_in_index: bool,
    pub preview_url: String,
    pub preview_width: u32,
    pub preview_height: u32,
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

impl BooruPostModel for MoebooruPost {
    fn id(&self) -> Cow<str> {
        self.id.to_string().into()
    }

    fn hash(&self) -> Option<ImageHash> {
        Some(ImageHash::MD5(self.md5.as_str().into()))
    }

    fn images(&self) -> Images {
        Images {
            original: Image::new(self.file_url.as_str())
                .size(self.width, self.height)
                .filesize(self.file_size)
                .into(),
            sample: Image::new(self.sample_url.as_str())
                .size(self.sample_width, self.sample_height)
                .filesize(self.sample_file_size)
                .into(),
            preview: Image::new(self.preview_url.as_str())
                .size(self.preview_width, self.preview_height)
                .into(),
        }
    }

    fn source_url(&self) -> Option<Cow<str>> {
        if self.source.len() > 0 {
            Some(self.source.as_str().into())
        } else {
            None
        }
    }

    fn tags(&self) -> Vec<String> {
        // TODO use Cow
        self.tags.split(" ").map(ToOwned::to_owned).collect()
    }

    fn created(&self) -> Option<Cow<str>> {
        Some(self.created_at.as_str().into())
    }
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
