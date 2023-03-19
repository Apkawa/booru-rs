use crate::client::generic::{BooruClient, BooruClientOptions, BooruOptionBuilder};

pub use self::model::{DanbooruPost, DanbooruRating, DanbooruSort};

pub mod model;

/// Client that sends requests to the Danbooru API to retrieve the data.
pub struct DanbooruClient {
    options: BooruClientOptions,
}

impl BooruClient for DanbooruClient {
    type PostModel = DanbooruPost;
    type PostResponse = Self::PostModel;
    type PostListResponse = Vec<Self::PostModel>;
    type Rating = DanbooruRating;
    type Order = DanbooruSort;
    const BASE_URL: &'static str = "https://danbooru.donmai.us";
    const PATH_POST_BY_ID: &'static str = "posts/{id}.json";
    const PATH_POST: &'static str = "posts.json?page={page}&tags={tags}&limit={limit}";

    fn with_options(options: BooruClientOptions) -> Self {
        DanbooruClient {
            options: options.into(),
        }
    }

    fn options(&'_ self) -> &'_ BooruClientOptions {
        &self.options
    }
}

impl BooruOptionBuilder for DanbooruClient {
    fn with_inner_options<F>(mut self, func: F) -> Self where F: FnOnce(BooruClientOptions) -> BooruClientOptions, Self: Sized {
        self.options = func(self.options);
        self
    }
}