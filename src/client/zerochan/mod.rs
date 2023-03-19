use crate::client::generic::{
    BooruClient, BooruClientBuilder, BooruClientBuilderOptions, BooruClientOptions,
};
use reqwest::Error;
/// https://www.zerochan.net/api
use std::collections::HashMap;

pub use self::model::{ZerochanListResponse, ZerochanPost, ZerochanRating, ZerochanSort};

pub mod model;

/// Client that sends requests to the Zerochan API to retrieve the data.
pub struct ZerochanClient {
    options: BooruClientOptions,
    order: Option<ZerochanSort>,
}

impl BooruClient<'_> for ZerochanClient {
    type Builder = ZerochanClientBuilder;
    type PostModel = ZerochanPost;
    type PostResponse = Self::PostModel;
    type PostListResponse = ZerochanListResponse;
    const PATH_POST_BY_ID: &'static str = "{id}?json";
    const PATH_POST: &'static str = "{tags}?json&p={page}&l={limit}";

    fn new(builder: Self::Builder) -> Self {
        ZerochanClient {
            options: builder.options.into(),
            order: builder.order,
        }
    }

    fn get_extra_query(&'_ self) -> HashMap<String, String> {
        let mut extra = HashMap::new();
        if let Some(order) = self.order.as_ref() {
            extra.insert("s".to_string(), order.to_string());
        }
        extra
    }

    fn options(&'_ self) -> &'_ BooruClientOptions {
        &self.options
    }

    fn get_with_page(&'_ self, page: Option<usize>) -> Result<Vec<Self::PostModel>, Error> {
        let url = self.url_posts(page);
        let extra_query = self.get_extra_query();
        let extra_query: Vec<_> = extra_query.iter().collect();
        let request = self
            .client()
            .get(url)
            .headers(self.options().headers.to_owned())
            .query(&extra_query);

        dbg!(&request);
        let response = request.send()?;
        dbg!(&response);
        // Ugly fix broken json
        let text = response.text().unwrap().replace("],\r\n  next: true", "]");

        let json = serde_json::from_str::<Self::PostListResponse>(&text);
        if json.is_err() {
            dbg!(&text);
            json.unwrap();
            unreachable!()
        }

        Ok(json.unwrap().into())
    }
}

/// Builder for [`ZerochanClient`]
#[derive(Default)]
pub struct ZerochanClientBuilder {
    options: BooruClientBuilderOptions,
    order: Option<ZerochanSort>,
}

impl BooruClientBuilder for ZerochanClientBuilder {
    type Client = ZerochanClient;
    type Rating = ZerochanRating;
    type Order = ZerochanSort;
    const BASE_URL: &'static str = "https://www.zerochan.net";

    fn with_inner_options<F>(mut self, func: F) -> Self
    where
        F: FnOnce(BooruClientBuilderOptions) -> BooruClientBuilderOptions,
    {
        self.options = func(self.options);
        self
    }

    fn new() -> ZerochanClientBuilder {
        ZerochanClientBuilder {
            options: BooruClientBuilderOptions::with_url(Self::BASE_URL),
            order: None,
        }
    }
    fn build(self) -> Self::Client
    where
        Self: Sized,
    {
        Self::Client::new(self)
    }

    fn order(mut self, order: Self::Order) -> Self
    where
        Self: Sized,
    {
        self.order = Some(order);
        self
    }
}
