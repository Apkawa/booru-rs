use reqwest::Error;
use crate::client::generic::{BooruClient, BooruClientBuilder, BooruClientBuilderOptions, BooruClientOptions};

use self::model::{MoebooruPost, MoebooruRating, MoebooruSort};

pub mod model;

/// Client that sends requests to the Danbooru API to retrieve the data.
pub struct MoebooruClient {
    options: BooruClientOptions,
}

impl BooruClient<'_> for MoebooruClient {
    type Builder = MoebooruClientBuilder;
    type PostModel = MoebooruPost;
    type PostResponse = Vec<Self::PostModel>;
    type PostListResponse = Vec<Self::PostModel>;
    const PATH_POST_BY_ID: &'static str = "post.json";
    const PATH_POST: &'static str = "post.json";

    fn new(options: BooruClientBuilderOptions) -> Self {
        MoebooruClient { options: options.into() }
    }

    fn options(&'_ self) -> &'_ BooruClientOptions {
        &self.options
    }
    fn get_by_id(&'_ self, id: u32) -> Result<Self::PostModel, Error> {
        let response = self
            .client()
            .get(self.url_post_by_id(id))
            .query(&[
                ("tags", format!("id:{id}").as_str()),
                ]
            )
            .send()?
            .json::<Self::PostResponse>()?;
        Ok(response.into())
    }
}

/// Builder for [`MoebooruClient`]
#[derive(Default)]
pub struct MoebooruClientBuilder {
    options: BooruClientBuilderOptions,
}

impl BooruClientBuilder for MoebooruClientBuilder {
    type Client = MoebooruClient;
    type Rating = MoebooruRating;
    type Order = MoebooruSort;
    const BASE_URL: &'static str = "https://konachan.com";

    fn with_inner_options<F>(mut self, func: F) -> Self
        where F: FnOnce(BooruClientBuilderOptions) -> BooruClientBuilderOptions {
        self.options = func(self.options);
        self
    }

    fn new() -> MoebooruClientBuilder {
        MoebooruClientBuilder {
            options: BooruClientBuilderOptions::with_url(Self::BASE_URL),
        }
    }

    fn build(self) -> Self::Client
        where Self: Sized {
        Self::Client::new(self.options)
    }
}
