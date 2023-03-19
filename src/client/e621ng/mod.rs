use crate::client::e621ng::model::{E621ngDetailResponse, E621ngListResponse};
/// https://e621.net/help/api
use crate::client::generic::{BooruClient, BooruClientOptions, BooruOptionBuilder};

pub use self::model::{E621ngPost, E621ngRating, E621ngSort};

pub mod model;

/// Client that sends requests to the Danbooru API to retrieve the data.
pub struct E621ngClient {
    options: BooruClientOptions,
}

impl BooruClient for E621ngClient {
    type PostModel = E621ngPost;
    type PostResponse = E621ngDetailResponse;
    type PostListResponse = E621ngListResponse;
    type Rating = E621ngRating;
    type Order = E621ngSort;
    const BASE_URL: &'static str = "https://e621.net";
    const PATH_POST_BY_ID: &'static str = "posts/{id}.json";
    const PATH_POST: &'static str = "posts.json?page={page}&tags={tags}&limit={limit}";

    fn with_options(options: BooruClientOptions) -> Self {
        E621ngClient {
            options: options.into(),
        }
    }

    fn options(&'_ self) -> &'_ BooruClientOptions {
        &self.options
    }
}


impl BooruOptionBuilder for E621ngClient {
    fn with_inner_options<F>(mut self, func: F) -> Self
    where
        F: FnOnce(BooruClientOptions) -> BooruClientOptions,
    {
        self.options = func(self.options);
        self
    }

}
