//! Models for Danbooru
use core::fmt;
use std::borrow::Cow;

use crate::client::generic::model::{Image, ImageHash, Images};
use crate::client::generic::BooruPostModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DanbooruPost {
    pub id: u32,
    pub created_at: String,
    pub updated_at: String,
    pub uploader_id: u32,
    pub approver_id: Option<u32>,
    pub tag_string: String,
    pub tag_string_general: String,
    pub tag_string_artist: String,
    pub tag_string_copyright: String,
    pub tag_string_character: String,
    pub tag_string_meta: String,
    pub rating: Option<DanbooruRating>,
    pub parent_id: Option<u32>,
    pub pixiv_id: Option<u32>,
    pub source: String,
    pub md5: Option<String>,
    pub file_url: Option<String>,
    pub large_file_url: Option<String>,
    pub preview_file_url: Option<String>,
    pub file_ext: String,
    pub file_size: usize,
    pub image_width: u32,
    pub image_height: u32,
    pub score: i32,
    pub up_score: i32,
    pub down_score: i32,
    pub fav_count: u32,
    pub tag_count_general: u32,
    pub tag_count_artist: u32,
    pub tag_count_copyright: u32,
    pub tag_count_character: u32,
    pub tag_count_meta: u32,
    pub last_comment_bumped_at: Option<String>,
    pub last_noted_at: Option<String>,
    pub has_large: bool,
    pub has_children: bool,
    pub has_visible_children: bool,
    pub has_active_children: bool,
    pub is_banned: bool,
    pub is_deleted: bool,
    pub is_flagged: bool,
    pub is_pending: bool,
    pub bit_flags: u32,
}

impl BooruPostModel for DanbooruPost {
    fn id(&self) -> Cow<str> {
        self.id.to_string().into()
    }

    fn hash(&self) -> Option<ImageHash> {
        self.md5.as_ref().map(|s| ImageHash::MD5(s))
    }

    fn images(&self) -> Images {
        Images {
            original: self.file_url.as_ref().map(|s| {
                Image::new(s.as_str())
                    .filesize(self.file_size)
                    .size(self.image_width, self.image_height)
                    .ext(self.file_ext.as_str())
            }),
            sample: self.large_file_url.as_ref().map(Image::new),
            preview: self.preview_file_url.as_ref().map(Image::new),
        }
    }

    fn source_url(&self) -> Option<Cow<str>> {
        Some(self.source.as_str().into())
    }

    fn tags(&self) -> Vec<String> {
        // TODO use Cow
        self.tag_string.split(" ").map(ToOwned::to_owned).collect()
    }

    fn character(&self) -> Vec<String> {
        self.tag_string_character
            .split(" ")
            .map(ToOwned::to_owned)
            .collect()
    }

    fn artist(&self) -> Option<Cow<str>> {
        Some(self.tag_string_artist.as_str().into())
    }

    fn created(&self) -> Option<Cow<str>> {
        Some(self.created_at.as_str().into())
    }
}

/// Post's rating. Check the [Danbooru's ratings wiki](https://danbooru.donmai.us/wiki_pages/howto:rate)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DanbooruRating {
    #[serde(rename = "e")]
    Explicit,
    #[serde(rename = "q")]
    Questionable,
    #[serde(rename = "s")]
    Sensitive,
    #[serde(rename = "g")]
    General,
}

impl From<DanbooruRating> for String {
    fn from(rating: DanbooruRating) -> String {
        rating.to_string()
    }
}

impl fmt::Display for DanbooruRating {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lowercase_tag = format!("{:?}", self).to_lowercase();
        write!(f, "{lowercase_tag}")
    }
}

#[derive(Debug, Clone)]
pub enum DanbooruSort {
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

impl fmt::Display for DanbooruSort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lowercase_tag = format!("{:?}", self).to_lowercase();
        write!(f, "{lowercase_tag}")
    }
}
