use crate::client::generic::{BooruClient, BooruClientOptions, BooruOptionBuilder};
use std::collections::HashMap;

pub use self::model::{
    PhilomenaDetailResponse, PhilomenaListResponse, PhilomenaPost, PhilomenaRating, PhilomenaSort,
};

pub mod model;

/// Client that sends requests to the Danbooru API to retrieve the data.
pub struct PhilomenaClient {
    options: BooruClientOptions,
}

impl BooruClient for PhilomenaClient {
    type PostModel = PhilomenaPost;
    type PostResponse = PhilomenaDetailResponse;
    type PostListResponse = PhilomenaListResponse;
    type Rating = PhilomenaRating;
    type Order = PhilomenaSort;
    const BASE_URL: &'static str = "https://derpibooru.org";
    const PATH_POST_BY_ID: &'static str = "api/v1/json/images/{id}";
    const PATH_POST: &'static str =
        "api/v1/json/search/images?q={tags}&page={page}&per_page={limit}";

    fn get_extra_query(&'_ self) -> HashMap<String, String> {
        let mut extra = HashMap::new();
        if let Some(order) = self.options().order.as_ref() {
            extra.insert("sf".to_string(), order.to_string());
        }
        extra
    }

    fn options(&'_ self) -> &'_ BooruClientOptions {
        &self.options
    }

    fn with_options(options: BooruClientOptions) -> Self {
        PhilomenaClient { options }
    }

    fn url_posts(&self) -> String {
        let options = self.options();
        let page = options.page;
        let mut tag_string = options.tags.join(" ");

        if let Some(random) = options.random.as_ref() {
            if *random {
                tag_string.push_str(" sort:random".to_string().as_str());
            }
        }
        if let Some(rating) = options.rating.as_ref() {
            tag_string.push_str(format!(" {rating}").as_str());
        }
        let tag_string = form_urlencoded::byte_serialize(tag_string.as_bytes());

        [(self.base_url()), Self::PATH_POST]
            .join("/")
            .replace("{page}", &page.to_string())
            .replace("{limit}", &self.options().limit.to_string())
            .replace("{tags}", &tag_string.collect::<String>())
    }
}

impl BooruOptionBuilder for PhilomenaClient {
    fn with_inner_options<F>(mut self, func: F) -> Self
    where
        F: FnOnce(BooruClientOptions) -> BooruClientOptions,
    {
        self.options = func(self.options);
        self
    }
}
