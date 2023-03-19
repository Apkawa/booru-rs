use reqwest::header;
use reqwest::header::HeaderValue;

/// Danbooru V1 old API
/// http://behoimi.org/help/api
/// http://behoimi.org/help/cheatsheet
use crate::client::generic::{BooruClient, BooruClientOptions, BooruOptionBuilder};

pub use self::model::{DanbooruPostV1, DanbooruRatingV1, DanbooruSortV1};

pub mod model;

/// Client that sends requests to the Danbooru API to retrieve the data.
pub struct DanbooruV1Client {
    options: BooruClientOptions,
}

impl BooruClient for DanbooruV1Client {
    type PostModel = DanbooruPostV1;
    type PostResponse = Vec<Self::PostModel>;
    type PostListResponse = Vec<Self::PostModel>;
    type Rating = DanbooruRatingV1;
    type Order = DanbooruSortV1;
    const BASE_URL: &'static str = "http://behoimi.org";
    const PATH_POST_BY_ID: &'static str = "post/index.json?tags=id:{id}";
    const PATH_POST: &'static str = "post/index.json?page={page}&tags={tags}&limit={limit}";

    fn with_options(options: BooruClientOptions) -> Self {
        DanbooruV1Client { options }.header(
            header::USER_AGENT,
            HeaderValue::from_static(
                "Mozilla/5.0 (X11; Linux x86_64) \
                    AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36",
            ),
        )
    }

    fn options(&'_ self) -> &'_ BooruClientOptions {
        &self.options
    }
}

impl BooruOptionBuilder for DanbooruV1Client {
    fn with_inner_options<F>(mut self, func: F) -> Self
    where
        F: FnOnce(BooruClientOptions) -> BooruClientOptions,
    {
        self.options = func(self.options);
        self
    }
}
