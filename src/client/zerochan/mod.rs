use crate::client::generic::{BooruClient, BooruClientOptions, BooruOptionBuilder};
use reqwest::Error;
/// https://www.zerochan.net/api
use std::collections::HashMap;

pub use self::model::{ZerochanListResponse, ZerochanPost, ZerochanRating, ZerochanSort};

pub mod model;

/// Client that sends requests to the Zerochan API to retrieve the data.
pub struct ZerochanClient {
    options: BooruClientOptions,
}

impl BooruClient for ZerochanClient {
    type PostModel = ZerochanPost;
    type PostResponse = Self::PostModel;
    type PostListResponse = ZerochanListResponse;
    type Rating = ZerochanRating;
    type Order = ZerochanSort;
    const BASE_URL: &'static str = "https://www.zerochan.net";
    const PATH_POST_BY_ID: &'static str = "{id}?json";
    const PATH_POST: &'static str = "{tags}?json&p={page}&l={limit}";

    fn with_options(options: BooruClientOptions) -> Self {
        ZerochanClient {
            options: options.into(),
        }
    }

    fn get_extra_query(&'_ self) -> HashMap<String, String> {
        let mut extra = HashMap::new();
        if let Some(order) = self.options().order.as_ref() {
            extra.insert("s".to_string(), order.to_string());
        }
        extra
    }

    fn options(&'_ self) -> &'_ BooruClientOptions {
        &self.options
    }

    fn get(&self) -> Result<Vec<Self::PostModel>, Error> {
        let url = self.url_posts();
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

impl BooruOptionBuilder for ZerochanClient {
    fn with_inner_options<F>(mut self, func: F) -> Self
    where
        F: FnOnce(BooruClientOptions) -> BooruClientOptions,
    {
        self.options = func(self.options);
        self
    }
}
