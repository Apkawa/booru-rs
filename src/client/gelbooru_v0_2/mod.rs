use crate::client::generic::{BooruClient, BooruClientBuilder, BooruClientBuilderOptions, BooruClientOptions};
use self::model::{GelbooruPostV0_2, GelbooruRating, GelbooruSort};

#[cfg(feature = "gelbooru")]
pub mod model;


/// Client that sends requests to the Gelbooru >=v0.2.0,<v0.2.5 API to retrieve the data.
pub struct GelbooruClientV0_2 {
    options: BooruClientOptions,
}

impl BooruClient<'_> for GelbooruClientV0_2 {
    type Builder = GelbooruClientBuilderV0_2;
    type PostModel = GelbooruPostV0_2;
    type PostResponse = Vec<Self::PostModel>;
    type PostListResponse = Vec<Self::PostModel>;
    const PATH_POST_BY_ID: &'static str = "index.php?page=dapi&s=post&q=index&json=1&id={id}";
    const PATH_POST: &'static str = "index.php?page=dapi&s=post&q=index&json=1&pid={page}";

    fn new(options: BooruClientBuilderOptions) -> Self {
        GelbooruClientV0_2 { options: options.into() }
    }

    fn options(&'_ self) -> &'_ BooruClientOptions {
        &self.options
    }
}

#[derive(Default)]
pub struct GelbooruClientBuilderV0_2 {
    options: BooruClientBuilderOptions,
}


impl BooruClientBuilder for GelbooruClientBuilderV0_2 {
    type Client = GelbooruClientV0_2;
    type Rating = GelbooruRating;
    type Order = GelbooruSort;

    const BASE_URL: &'static str = "https://safebooru.org";

    fn new() -> Self {
        GelbooruClientBuilderV0_2 {
            options: BooruClientBuilderOptions::with_url(Self::BASE_URL)
        }
    }

    fn build(self) -> Self::Client
        where Self: Sized {
        Self::Client::new(self.options)
    }

    fn random(self, random: bool) -> Self {
        if random {
            self.tag("sort:random".to_string())
        } else {
            self
        }
    }
    // https://gelbooru.com/index.php?page=help&topic=cheatsheet
    fn order(self, order: Self::Order) -> Self
        where
            Self: Sized
    {
        self.tag(format!("sort:{}", order))
    }

    fn with_inner_options<F>(mut self, func: F) -> Self
        where F: FnOnce(BooruClientBuilderOptions) -> BooruClientBuilderOptions {
        self.options = func(self.options);
        self
    }
}


