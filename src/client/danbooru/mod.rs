use reqwest::blocking::{Client, ClientBuilder};
use reqwest::{IntoUrl, Proxy};

use crate::model::danbooru::{DanbooruPost, DanbooruRating, DanbooruSort};
use crate::utils::general::get_headers;

/// Client that sends requests to the Danbooru API to retrieve the data.
pub struct DanbooruClient;

impl DanbooruClient {
    pub fn builder() -> DanbooruClientBuilder {
        DanbooruClientBuilder::new()
    }
}

/// Builder for [`DanbooruClient`]
#[derive(Default)]
pub struct DanbooruClientBuilder {
    client_builder: Option<ClientBuilder>,
    client: Option<Client>,
    key: Option<String>,
    user: Option<String>,
    tags: Vec<String>,
    limit: u32,
    url: String,
}

impl DanbooruClientBuilder {
    pub fn new() -> DanbooruClientBuilder {
        DanbooruClientBuilder {
            client_builder: Some(ClientBuilder::default()),
            client: None,
            key: None,
            user: None,
            tags: vec![],
            limit: 100,
            url: "https://danbooru.donmai.us".to_string(),
        }
    }
    pub fn build(mut self) -> Self {
        self.client = Some(
            self.client_builder
                .take()
                .unwrap()
                .default_headers(get_headers())
                .build()
                .unwrap(),
        );

        self
    }

    pub fn client(&self) -> &Client {
        self.client.as_ref().unwrap()
    }

    pub fn proxy<I: IntoUrl>(mut self, proxy: Option<I>) -> Self {
        if let Some(proxy) = proxy {
            let proxy = Proxy::all(proxy).unwrap();
            self.client_builder = Some(self.client_builder.unwrap().proxy(proxy));
        } else {
            self.client_builder = Some(self.client_builder.unwrap().no_proxy());
        };
        self
    }
    /// Set the API key and User for the requests (optional)
    pub fn set_credentials(mut self, key: String, user: String) -> Self {
        self.key = Some(key);
        self.user = Some(user);
        self
    }

    /// Add a tag to the query
    pub fn tag<S: Into<String>>(mut self, tag: S) -> Self {
        if self.tags.len() > 1 {
            panic!("Danbooru only allows 2 tags per query");
        }
        self.tags.push(tag.into());
        self
    }

    /// Add a [`DanbooruRating`] to the query
    pub fn rating(mut self, rating: DanbooruRating) -> Self {
        self.tags.push(format!("rating:{}", rating));
        self
    }

    /// Set how many posts you want to retrieve (100 is the default and maximum)
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = limit;
        self
    }

    /// Retrieves the posts in a random order
    pub fn random(mut self, random: bool) -> Self {
        if random {
            self.tags.push("order:random".into());
        }
        self
    }

    /// Add a [`DanbooruSort`] to the query
    pub fn sort(mut self, order: DanbooruSort) -> Self {
        self.tags.push(format!("order:{}", order));
        self
    }

    /// Blacklist a tag from the query
    pub fn blacklist_tag<S: Into<String>>(mut self, tag: S) -> Self {
        self.tags.push(format!("-{}", tag.into()));
        self
    }

    /// Change the default url for the client
    pub fn default_url(mut self, url: &str) -> Self {
        self.url = url.into();
        self
    }

    /// Directly get a post by its unique Id
    pub fn get_by_id(&self, id: u32) -> Result<DanbooruPost, reqwest::Error> {
        let url = self.url.as_str();
        let response = self
            .client()
            .get(format!("{url}/posts/{id}.json"))
            .send()?
            .json::<DanbooruPost>()?;
        Ok(response)
    }

    /// Pack the [`DanbooruClientBuilder`] and sent the request to the API to retrieve the posts
    pub fn get(&self) -> Result<Vec<DanbooruPost>, reqwest::Error> {
        let tag_string = self.tags.join(" ");
        let url = self.url.as_str();
        let response = self
            .client()
            .get(format!("{url}/posts.json"))
            .query(&[
                ("limit", self.limit.to_string().as_str()),
                ("tags", &tag_string),
            ])
            .send()?
            .json::<Vec<DanbooruPost>>()?;

        Ok(response)
    }
}
