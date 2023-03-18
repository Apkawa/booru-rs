use std::collections::HashMap;
use crate::client::generic::{BooruClient, BooruClientBuilder, BooruClientBuilderOptions, BooruClientOptions};

pub use self::model::{PhilomenaDetailResponse, PhilomenaListResponse, PhilomenaPost, PhilomenaRating, PhilomenaSort};

pub mod model;

/// Client that sends requests to the Danbooru API to retrieve the data.
pub struct PhilomenaClient {
    options: BooruClientOptions,
    order: Option<PhilomenaSort>,
}

impl BooruClient<'_> for PhilomenaClient {
    type Builder = PhilomenaClientBuilder;
    type PostModel = PhilomenaPost;
    type PostResponse = PhilomenaDetailResponse;
    type PostListResponse = PhilomenaListResponse;
    const PATH_POST_BY_ID: &'static str = "api/v1/json/images/{id}";
    const PATH_POST: &'static str = "api/v1/json/search/images?q={tags}&page={page}&per_page={limit}";

    fn new(builder: Self::Builder) -> Self {
        PhilomenaClient {
            options: builder.options.into(),
            order: builder.order,
        }
    }
    fn get_extra_query(&'_ self) -> HashMap<String, String> {
        let mut extra = HashMap::new();
        if let Some(order) = self.order.as_ref() {
            extra.insert("sf".to_string(), order.to_string());
        }
        extra
    }

    fn options(&'_ self) -> &'_ BooruClientOptions {
        &self.options
    }
}

/// Builder for [`PhilomenaClient`]
#[derive(Default)]
pub struct PhilomenaClientBuilder {
    options: BooruClientBuilderOptions,
    order: Option<PhilomenaSort>,
}

impl BooruClientBuilder for PhilomenaClientBuilder {
    type Client = PhilomenaClient;
    type Rating = PhilomenaRating;
    type Order = PhilomenaSort;
    const BASE_URL: &'static str = "https://derpibooru.org";

    fn with_inner_options<F>(mut self, func: F) -> Self
        where F: FnOnce(BooruClientBuilderOptions) -> BooruClientBuilderOptions {
        self.options = func(self.options);
        self
    }

    fn new() -> PhilomenaClientBuilder {
        PhilomenaClientBuilder {
            options: BooruClientBuilderOptions::with_url(Self::BASE_URL),
            order: None,
        }
    }
    fn build(self) -> Self::Client
        where Self: Sized {
        Self::Client::new(self)
    }

    fn rating(self, rating: Self::Rating) -> Self where Self: Sized {
        self.tag(rating.to_string())
    }

    fn order(mut self, order: Self::Order) -> Self where Self: Sized {
        self.order = Some(order);
        self
    }
}
