use std::collections::HashMap;
use std::fmt::Display;

use reqwest;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{IntoUrl, Proxy};
use serde::de::DeserializeOwned;

use crate::utils::general::get_default_headers;

pub mod model;

use crate::client::generic::model::BooruPostModelSetUrl;
pub use model::BooruPostModel;

#[derive(Debug, Clone)]
pub struct BooruClientOptions {
    pub(crate) url: Option<String>,
    pub(crate) proxy: Option<Proxy>,
    pub(crate) tags: Vec<String>,
    pub(crate) order: Option<String>,
    pub(crate) rating: Option<String>,
    pub(crate) random: Option<bool>,
    pub(crate) page: u32,
    pub(crate) limit: u32,
    pub(crate) headers: HeaderMap,
}

impl Default for BooruClientOptions {
    fn default() -> Self {
        BooruClientOptions {
            proxy: None,
            headers: get_default_headers(),
            url: None,
            tags: vec![],
            order: None,
            rating: None,
            random: None,
            page: 1,
            limit: 100,
        }
    }
}

impl BooruClientOptions {
    pub fn client(&self) -> reqwest::blocking::Client {
        let mut client_builder = reqwest::blocking::ClientBuilder::new();
        if let Some(proxy) = self.proxy.as_ref() {
            client_builder = client_builder.proxy(proxy.to_owned());
        } else {
            client_builder = client_builder.no_proxy();
        }
        client_builder.build().unwrap()
    }
}

#[must_use]
pub trait BooruClient {
    type PostModel: DeserializeOwned + BooruPostModel + BooruPostModelSetUrl;
    type PostResponse: DeserializeOwned + Into<Self::PostModel>;
    type PostListResponse: DeserializeOwned + Into<Vec<Self::PostModel>>;
    type Rating: Display;
    type Order: Display;

    const BASE_URL: &'static str;
    const PATH_POST_BY_ID: &'static str;
    const PATH_POST: &'static str;

    fn new() -> Self
    where
        Self: Sized,
    {
        Self::with_options(BooruClientOptions::default())
    }
    fn with_options(options: BooruClientOptions) -> Self;

    fn options(&self) -> &BooruClientOptions;

    fn client(&self) -> reqwest::blocking::Client {
        self.options().client()
    }

    fn base_url(&self) -> &str {
        self.options().url.as_deref().unwrap_or(Self::BASE_URL)
    }

    fn url_post_by_id(&self, id: &str) -> String {
        let path = Self::PATH_POST_BY_ID.replace("{id}", id);
        [(self.base_url()), path.as_str()].join("/")
    }

    fn url_posts(&self) -> String {
        let options = self.options();
        let page = options.page;
        let mut tag_string = options.tags.join(" ");

        if let Some(order) = options.order.as_ref() {
            tag_string.push_str(format!(" order:{order}").as_str());
        }
        if let Some(random) = options.random.as_ref() {
            if *random {
                tag_string.push_str(" order:random".to_string().as_str());
            }
        }
        if let Some(rating) = options.rating.as_ref() {
            tag_string.push_str(format!(" rating:{rating}").as_str());
        }
        let tag_string = form_urlencoded::byte_serialize(tag_string.as_bytes());

        [(self.base_url()), Self::PATH_POST]
            .join("/")
            .replace("{page}", &page.to_string())
            .replace("{limit}", &self.options().limit.to_string())
            .replace("{tags}", &tag_string.collect::<String>())
    }

    /// Directly get a post by its unique Id
    fn get_by_id<I: Display>(&self, id: I) -> Result<Self::PostModel, reqwest::Error> {
        let request = self
            .client()
            .get(self.url_post_by_id(&id.to_string()))
            .headers(self.options().headers.to_owned());
        dbg!(&request);
        let response = request.send()?;
        dbg!(&response);
        let text = response.text().unwrap();
        let json = serde_json::from_str::<Self::PostResponse>(&text).expect(&text);
        Ok(json.into().set_base_url(self.base_url()))
    }

    fn get_extra_query(&self) -> HashMap<String, String> {
        HashMap::new()
    }

    /// Get first page
    fn get(&self) -> Result<Vec<Self::PostModel>, reqwest::Error> {
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
        let text = response.text().unwrap();
        let json = serde_json::from_str::<Self::PostListResponse>(&text).expect(&text);
        Ok(json.into().set_base_url(self.base_url()))
    }
}

#[must_use]
pub trait BooruOptionBuilder {
    fn with_inner_options<F>(self, func: F) -> Self
    where
        F: FnOnce(BooruClientOptions) -> BooruClientOptions,
        Self: Sized;

    fn proxy<I: IntoUrl>(self, proxy: Option<I>) -> Self
    where
        Self: Sized,
    {
        self.with_inner_options(move |mut options| {
            options.proxy = proxy.map(|proxy| Proxy::all(proxy).unwrap());
            options
        })
    }

    fn header<K: Into<HeaderName>, V: Into<HeaderValue>>(self, key: K, value: V) -> Self
    where
        Self: Sized,
    {
        self.with_inner_options(move |mut options| {
            options.headers.insert(key.into(), value.into());
            options
        })
    }
    /// Change the default url for the client
    fn url(self, url: &str) -> Self
    where
        Self: Sized,
    {
        self.with_inner_options(move |mut options| {
            options.url = Some(url.to_string());
            options
        })
    }

    /// Set how many posts you want to retrieve (100 is the default and maximum)
    fn limit(self, limit: u32) -> Self
    where
        Self: Sized,
    {
        self.with_inner_options(move |mut options| {
            options.limit = limit;
            options
        })
    }

    fn page(self, page: u32) -> Self
    where
        Self: Sized,
    {
        self.with_inner_options(move |mut options| {
            options.page = page;
            options
        })
    }

    /// Add a tag to the query
    fn tag<S: Into<String>>(self, tag: S) -> Self
    where
        Self: Sized,
    {
        self.with_inner_options(move |mut options| {
            options.tags.push(tag.into());
            options
        })
    }

    /// Add a [`Self::Rating`] to the query
    fn rating<R: Display>(self, rating: R) -> Self
    where
        Self: Sized,
    {
        self.with_inner_options(move |mut options| {
            options.rating = Some(format!("{}", rating));
            options
        })
    }

    /// Retrieves the posts in a random order
    fn random(self, random: bool) -> Self
    where
        Self: Sized,
    {
        self.with_inner_options(move |mut options| {
            options.random = random.into();
            options
        })
    }

    /// Add a [`Self::Order`] to the query
    fn order<O: Display>(self, order: O) -> Self
    where
        Self: Sized,
    {
        self.with_inner_options(move |mut options| {
            options.order = Some(format!("{}", order));
            options
        })
    }

    /// Blacklist a tag from the query
    fn blacklist_tag<S: Into<String>>(self, tag: S) -> Self
    where
        Self: Sized,
    {
        self.tag(format!("-{}", tag.into()))
    }
}
