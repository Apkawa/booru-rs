use crate::client::e621ng::model::{E621ngDetailResponse, E621ngListResponse};
/// https://e621.net/help/api
use crate::client::generic::{
    BooruClient, BooruClientBuilder, BooruClientBuilderOptions, BooruClientOptions,
};

pub use self::model::{E621ngPost, E621ngRating, E621ngSort};

pub mod model;

/// Client that sends requests to the Danbooru API to retrieve the data.
pub struct E621ngClient {
    options: BooruClientOptions,
}

impl BooruClient<'_> for E621ngClient {
    type Builder = E621ngClientBuilder;
    type PostModel = E621ngPost;
    type PostResponse = E621ngDetailResponse;
    type PostListResponse = E621ngListResponse;
    const PATH_POST_BY_ID: &'static str = "posts/{id}.json";
    const PATH_POST: &'static str = "posts.json?page={page}&tags={tags}&limit={limit}";

    fn new(builder: Self::Builder) -> Self {
        E621ngClient {
            options: builder.options.into(),
        }
    }

    fn options(&'_ self) -> &'_ BooruClientOptions {
        &self.options
    }
}

/// Builder for [`E621ngClient`]
#[derive(Default)]
pub struct E621ngClientBuilder {
    options: BooruClientBuilderOptions,
}

impl BooruClientBuilder for E621ngClientBuilder {
    type Client = E621ngClient;
    type Rating = E621ngRating;
    type Order = E621ngSort;
    const BASE_URL: &'static str = "https://e621.net";

    fn with_inner_options<F>(mut self, func: F) -> Self
    where
        F: FnOnce(BooruClientBuilderOptions) -> BooruClientBuilderOptions,
    {
        self.options = func(self.options);
        self
    }

    fn new() -> E621ngClientBuilder {
        E621ngClientBuilder {
            options: BooruClientBuilderOptions::with_url(Self::BASE_URL),
        }
    }

    fn build(self) -> Self::Client
    where
        Self: Sized,
    {
        Self::Client::new(self)
    }
}
