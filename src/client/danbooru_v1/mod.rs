use reqwest::header;
use reqwest::header::{HeaderName, HeaderValue};

/// Danbooru V1 old API
/// http://behoimi.org/help/api
/// http://behoimi.org/help/cheatsheet
use crate::client::generic::{BooruClient, BooruClientBuilder, BooruClientBuilderOptions, BooruClientOptions};

pub use self::model::{DanbooruPostV1, DanbooruRatingV1, DanbooruSortV1};

pub mod model;

/// Client that sends requests to the Danbooru API to retrieve the data.
pub struct DanbooruClientV1 {
    options: BooruClientOptions,
}

impl BooruClient<'_> for DanbooruClientV1 {
    type Builder = DanbooruClientBuilderV1;
    type PostModel = DanbooruPostV1;
    type PostResponse = Vec<Self::PostModel>;
    type PostListResponse = Vec<Self::PostModel>;
    const PATH_POST_BY_ID: &'static str = "post/index.json?tags=id:{id}";
    const PATH_POST: &'static str = "post/index.json?page={page}&tags={tags}&limit={limit}";

    fn new(builder: Self::Builder) -> Self {
        DanbooruClientV1 { options: builder.options.into() }
    }

    fn options(&'_ self) -> &'_ BooruClientOptions {
        &self.options
    }
}

/// Builder for [`DanbooruClientV1`]
#[derive(Default)]
pub struct DanbooruClientBuilderV1 {
    options: BooruClientBuilderOptions,
}

impl BooruClientBuilder for DanbooruClientBuilderV1 {
    type Client = DanbooruClientV1;
    type Rating = DanbooruRatingV1;
    type Order = DanbooruSortV1;
    const BASE_URL: &'static str = "http://behoimi.org";

    fn with_inner_options<F>(mut self, func: F) -> Self
        where F: FnOnce(BooruClientBuilderOptions) -> BooruClientBuilderOptions {
        self.options = func(self.options);
        self
    }

    fn new() -> DanbooruClientBuilderV1 {
        DanbooruClientBuilderV1 {
            options: BooruClientBuilderOptions::with_url(Self::BASE_URL),
        }
            .header(header::USER_AGENT,
                    HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) \
                    AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36"))
    }

    fn build(self) -> Self::Client
        where Self: Sized {
        Self::Client::new(self)
    }
}
