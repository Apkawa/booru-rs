use std::borrow::Cow;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ImageHash<'a> {
    MD5(&'a str),
    // SHA1(String),
    // SHA2(String),
    // SHA3(String),
    SHA512(&'a str),
}

impl Display for ImageHash<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use ImageHash::*;
        let s = match self {
            MD5(s) | SHA512(s) => s,
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Default)]
pub struct ImageSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Default)]
pub struct Image<'a> {
    pub url: Cow<'a, str>,
    pub filesize: Option<usize>,
    pub size: Option<ImageSize>,
    pub ext: Option<Cow<'a, str>>,
}

impl<'a> Image<'a> {
    pub fn new<U: Into<Cow<'a, str>>>(url: U) -> Image<'a> {
        Image {
            url: url.into(),
            ..Image::default()
        }
    }
    pub fn filesize(mut self, size: usize) -> Self {
        self.filesize = Some(size);
        self
    }
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.size = Some(ImageSize { width, height });
        self
    }
    pub fn ext<E: Into<Cow<'a, str>>>(mut self, ext: E) -> Self {
        self.ext = Some(ext.into());
        self
    }
}

impl<'a, I> From<I> for Image<'a>
    where
        I: Into<&'a str>,
{
    fn from(value: I) -> Image<'a> {
        Image::new(value.into())
    }
}

#[derive(Debug, Default)]
pub struct Images<'a> {
    pub original: Option<Image<'a>>,
    pub sample: Option<Image<'a>>,
    pub preview: Option<Image<'a>>,
}

pub trait BooruPostModel {
    fn id(&self) -> Cow<str>;
    fn hash(&self) -> Option<ImageHash>;
    fn images(&self) -> Images;
    fn source_url(&self) -> Option<Cow<str>>;
    fn post_url(&self) -> Option<Cow<str>> {
        None
    }
    fn tags(&self) -> Vec<Cow<str>>;

    fn character(&self) -> Vec<Cow<str>> {
        Vec::new()
    }
    fn artist(&self) -> Option<Cow<str>> {
        None
    }
    fn created(&self) -> Option<Cow<str>> {
        None
    }
}

pub trait BooruPostModelSetUrl {
    fn set_base_url<I: Into<String> + Clone>(self, _url: I) -> Self
        where Self: Sized
    {
        self
    }
}

impl<T: BooruPostModelSetUrl> BooruPostModelSetUrl for Vec<T> {
    fn set_base_url<I: Into<String> + Clone>(self, url: I) -> Self
        where Self: Sized
    {
        self.into_iter()
            .map(|e| e.set_base_url(url.clone().into()))
            .collect()
    }
}