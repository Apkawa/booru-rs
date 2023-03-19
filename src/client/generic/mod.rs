use std::collections::HashMap;
use std::fmt::Display;

use reqwest;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{IntoUrl, Proxy};
use serde::de::DeserializeOwned;

use crate::utils::general::get_default_headers;

pub mod model;
pub use model::BooruPostModel;

#[derive(Default)]
pub struct BooruClientOptions {
    pub(crate) client: reqwest::blocking::Client,
    pub(crate) url: String,
    pub(crate) tags: Vec<String>,
    pub(crate) limit: u32,
    pub(crate) headers: HeaderMap,
}

pub trait BooruClient<'a> {
    type Builder: BooruClientBuilder + Default + Sized;
    type PostModel: DeserializeOwned + BooruPostModel;
    type PostResponse: DeserializeOwned + Into<Self::PostModel>;
    type PostListResponse: DeserializeOwned + Into<Vec<Self::PostModel>>;

    const PATH_POST_BY_ID: &'static str;
    const PATH_POST: &'static str;

    fn builder() -> Self::Builder {
        Self::Builder::new()
    }

    fn new(builder: Self::Builder) -> Self;

    fn options(&'a self) -> &'a BooruClientOptions;

    fn client(&'a self) -> &'a reqwest::blocking::Client {
        &self.options().client
    }

    fn url_post_by_id(&'a self, id: &str) -> String {
        let path = Self::PATH_POST_BY_ID.replace("{id}", id);
        [&self.options().url, path.as_str()].join("/")
    }

    fn url_posts(&'a self, page: Option<usize>) -> String {
        let page = page.unwrap_or(1);
        let tag_string = self.options().tags.join(" ");
        let tag_string = form_urlencoded::byte_serialize(tag_string.as_bytes());

        [&self.options().url, Self::PATH_POST]
            .join("/")
            .replace("{page}", &page.to_string())
            .replace("{limit}", &self.options().limit.to_string())
            .replace("{tags}", &tag_string.collect::<String>())
    }

    /// Directly get a post by its unique Id
    fn get_by_id<'b, I: Display>(&'a self, id: I) -> Result<Self::PostModel, reqwest::Error> {
        let request = self
            .client()
            .get(self.url_post_by_id(&id.to_string()))
            .headers(self.options().headers.to_owned());
        dbg!(&request);
        let response = request.send()?;
        dbg!(&response);
        let text = response.text().unwrap();
        let json = serde_json::from_str::<Self::PostResponse>(&text);
        if json.is_err() {
            dbg!(&text);
            json.unwrap();
            unreachable!()
        }
        Ok(json.unwrap().into())
    }

    fn get_extra_query(&'a self) -> HashMap<String, String> {
        HashMap::new()
    }

    fn get_with_page(
        &'a self,
        page: Option<usize>,
    ) -> Result<Vec<Self::PostModel>, reqwest::Error> {
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
        let text = response.text().unwrap();
        let json = serde_json::from_str::<Self::PostListResponse>(&text);
        if json.is_err() {
            dbg!(&text);
            json.unwrap();
            unreachable!()
        }

        Ok(json.unwrap().into())
    }

    /// Get first page
    fn get(&'a self) -> Result<Vec<Self::PostModel>, reqwest::Error> {
        self.get_with_page(None)
    }
}

pub struct BooruClientBuilderOptions {
    pub client_builder: reqwest::blocking::ClientBuilder,
    pub proxy: Option<Proxy>,
    pub headers: HeaderMap,
    pub url: String,
    pub tags: Vec<String>,
    pub limit: u32,
}

impl BooruClientBuilderOptions {
    pub fn with_url<U: Into<String>>(url: U) -> BooruClientBuilderOptions {
        BooruClientBuilderOptions {
            url: url.into(),
            ..BooruClientBuilderOptions::default()
        }
    }
}

impl Default for BooruClientBuilderOptions {
    fn default() -> Self {
        BooruClientBuilderOptions {
            client_builder: Default::default(),
            proxy: None,
            headers: get_default_headers(),
            url: "".to_string(),
            tags: vec![],
            limit: 100,
        }
    }
}

impl From<BooruClientBuilderOptions> for BooruClientOptions {
    fn from(value: BooruClientBuilderOptions) -> Self {
        dbg!(&value.headers);
        let mut client_builder = value.client_builder;
        if let Some(proxy) = value.proxy {
            client_builder = client_builder.proxy(proxy);
        } else {
            client_builder = client_builder.no_proxy();
        }
        BooruClientOptions {
            client: client_builder.build().unwrap(),
            headers: value.headers.to_owned(),
            url: value.url.to_owned(),
            tags: value.tags.to_owned(),
            limit: value.limit,
        }
    }
}

#[must_use]
pub trait BooruClientBuilder {
    type Client: for<'a> BooruClient<'a>;
    type Rating: Display;
    type Order: Display;

    const BASE_URL: &'static str;
    const MAX_TAGS_LENGTH: usize = 2;

    fn new() -> Self
    where
        Self: Sized + Default,
    {
        Self::default()
    }
    fn build(self) -> Self::Client;

    fn with_inner_options<F>(self, func: F) -> Self
    where
        F: FnOnce(BooruClientBuilderOptions) -> BooruClientBuilderOptions;

    fn proxy<I: IntoUrl>(self, proxy: Option<I>) -> Self
    where
        Self: Sized,
    {
        self.with_inner_options(move |mut options| {
            options.proxy = if let Some(proxy) = proxy {
                Some(Proxy::all(proxy).unwrap())
            } else {
                None
            };
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
    fn default_url(self, url: &str) -> Self
    where
        Self: Sized,
    {
        self.with_inner_options(move |mut options| {
            options.url = url.to_string();
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

    /// Add a tag to the query
    fn tag<S: Into<String>>(self, tag: S) -> Self
    where
        Self: Sized,
    {
        self.with_inner_options(move |mut options| {
            if options.tags.len() > Self::MAX_TAGS_LENGTH {
                panic!("booru only allows 2 tags per query");
            }
            options.tags.push(tag.into());
            options
        })
    }

    /// Add a [`Self::Rating`] to the query
    fn rating(self, rating: Self::Rating) -> Self
    where
        Self: Sized,
    {
        self.tag(format!("rating:{}", rating))
    }

    /// Retrieves the posts in a random order
    fn random(self, random: bool) -> Self
    where
        Self: Sized,
    {
        if random {
            self.tag("order:random".to_string())
        } else {
            self
        }
    }

    /// Add a [`Self::Order`] to the query
    fn order(self, order: Self::Order) -> Self
    where
        Self: Sized,
    {
        self.tag(format!("order:{}", order))
    }

    /// Blacklist a tag from the query
    fn blacklist_tag<S: Into<String>>(self, tag: S) -> Self
    where
        Self: Sized,
    {
        self.tag(format!("-{}", tag.into()))
    }
}
