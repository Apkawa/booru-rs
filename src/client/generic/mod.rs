use crate::model::gelbooru::*;

pub trait BooruClient<A> {
    fn builder() -> A;
}

pub trait BooruBuilder<A, B> {
    fn new() -> Self;

    fn set_credentials(self, key: String, user: String) -> Self;

    fn tag<S: Into<String>>(self, tag: S) -> Self;

    fn rating(self, rating: A) -> Self;

    fn limit(self, limit: u32) -> Self;

    fn random(self, random: bool) -> Self;

    fn sort(self, order: B) -> Self;

    fn blacklist_tag<S: Into<String>>(self, tag: S) -> Self;

    fn default_url(self, url: &str) -> Self;

    fn get_by_id(&self, id: u32) -> Result<GelbooruPost, reqwest::Error>;

    fn get(&self) -> Result<Vec<GelbooruPost>, reqwest::Error>;
}
