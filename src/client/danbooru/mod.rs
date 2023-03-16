use std::ops::Deref;
use reqwest::blocking::{Client, ClientBuilder};
use reqwest::{IntoUrl, Proxy};
use crate::client::generic::{BooruClient, BooruClientBuilder, BooruClientBuilderOptions, BooruClientOptions};

use crate::model::danbooru::{DanbooruPost, DanbooruRating, DanbooruSort};
use crate::utils::general::get_headers;

/// Client that sends requests to the Danbooru API to retrieve the data.
pub struct DanbooruClient {
    options: BooruClientOptions,
}

impl BooruClient<'_> for DanbooruClient {
    type Builder = DanbooruClientBuilder;
    type PostModel = DanbooruPost;
    type PostListModel = Vec<Self::PostModel>;
    const PATH_POST_BY_ID: &'static str = "posts/{id}.json";
    const PATH_POST: &'static str = "posts.json";

    fn new(options: BooruClientBuilderOptions) -> Self {
        DanbooruClient { options: options.into() }
    }

    fn options(&'_ self) -> &'_ BooruClientOptions {
        &self.options
    }
}

/// Builder for [`DanbooruClient`]
#[derive(Default)]
pub struct DanbooruClientBuilder {
    options: BooruClientBuilderOptions,
}

impl BooruClientBuilder for DanbooruClientBuilder {
    type Client = DanbooruClient;
    type Rating = DanbooruRating;
    type Order = DanbooruSort;
    const BASE_URL: &'static str = "https://danbooru.donmai.us";

    fn with_inner_options<F>(mut self, func: F) -> Self
        where F: FnOnce(BooruClientBuilderOptions) -> BooruClientBuilderOptions {
        self.options = func(self.options);
        self
    }

    fn new() -> DanbooruClientBuilder {
        DanbooruClientBuilder {
            options: BooruClientBuilderOptions::with_url(Self::BASE_URL),
        }
    }

    fn build(self) -> Self::Client
        where Self: Sized {
        Self::Client::new(self.options)
    }
}
