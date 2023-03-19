use crate::client::generic::{BooruClient, BooruClientOptions, BooruOptionBuilder};

use self::model::{MoebooruPost, MoebooruRating, MoebooruSort};

pub mod model;

/// Client that sends requests to the Danbooru API to retrieve the data.
pub struct MoebooruClient {
    options: BooruClientOptions,
}

impl BooruClient for MoebooruClient {
    type PostModel = MoebooruPost;
    type PostResponse = Vec<Self::PostModel>;
    type PostListResponse = Vec<Self::PostModel>;
    type Rating = MoebooruRating;
    type Order = MoebooruSort;
    const BASE_URL: &'static str = "https://konachan.com";
    const PATH_POST_BY_ID: &'static str = "post.json?tags=id:{id}";
    const PATH_POST: &'static str = "post.json?page={page}&tags={tags}&limit={limit}";

    fn with_options(options: BooruClientOptions) -> Self {
        MoebooruClient {
            options: options.into(),
        }
    }

    fn options(&'_ self) -> &'_ BooruClientOptions {
        &self.options
    }
}

impl BooruOptionBuilder for MoebooruClient {

    fn with_inner_options<F>(mut self, func: F) -> Self
    where
        F: FnOnce(BooruClientOptions) -> BooruClientOptions,
    {
        self.options = func(self.options);
        self
    }
}
