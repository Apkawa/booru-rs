//! Models for Gelbooru
use core::fmt;
use std::borrow::Cow;

use crate::client::generic::model::{Image, ImageHash, Images};
use crate::client::generic::BooruPostModel;
use serde::{Deserialize, Serialize};

/// Individual post from [`GelbooruResponse`]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GelbooruPostV0_2 {
    /// The ID of the post
    pub id: u32,
    /// Post's score
    pub score: Option<u32>,
    /// Post's image width
    pub width: u32,
    /// Post's image height
    pub height: u32,
    /// Post's image md5
    pub hash: String,
    /// Post's tags
    pub tags: String,
    /// Post's rating
    pub directory: String,
    pub image: String,
    pub change: i64,
    pub owner: String,
    pub parent_id: i64,
    pub rating: String,
    pub sample: bool,
    pub sample_height: u32,
    pub sample_width: u32,
}

impl BooruPostModel for GelbooruPostV0_2 {
    fn id(&self) -> Cow<str> {
        self.id.to_string().into()
    }

    fn hash(&self) -> Option<ImageHash> {
        Some(ImageHash::MD5(self.hash.as_str().into()))
    }

    fn images(&self) -> Images {
        // TODO get image url (different site have different url)
        Images {
            original: Image::new(self.image.as_str())
                .size(self.width, self.height)
                .into(),
            ..Images::default()
        }
    }

    fn source_url(&self) -> Option<Cow<str>> {
        None
    }

    fn tags(&self) -> Vec<String> {
        // TODO use Cow
        self.tags.split(" ").map(ToOwned::to_owned).collect()
    }

    fn created(&self) -> Option<Cow<str>> {
        // TODO
        None
    }
}

impl From<Vec<GelbooruPostV0_2>> for GelbooruPostV0_2 {
    fn from(value: Vec<GelbooruPostV0_2>) -> Self {
        value[0].to_owned()
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
