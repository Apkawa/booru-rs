use self::model::{GelbooruPost, GelbooruRating, GelbooruResponse, GelbooruSort};
use crate::client::generic::{
    BooruClient, BooruClientBuilder, BooruClientBuilderOptions, BooruClientOptions,
};

#[cfg(feature = "gelbooru")]
pub mod model;

/// Client that sends requests to the Gelbooru >=v0.2.5 API to retrieve the data.
pub struct GelbooruClient {
    options: BooruClientOptions,
}

impl BooruClient<'_> for GelbooruClient {
    type Builder = GelbooruClientBuilder;
    type PostModel = GelbooruPost;
    type PostResponse = GelbooruResponse;
    type PostListResponse = GelbooruResponse;
    const PATH_POST_BY_ID: &'static str = "index.php?page=dapi&s=post&q=index&json=1&id={id}";
    const PATH_POST: &'static str =
        "index.php?page=dapi&s=post&q=index&json=1&pid={page}&tags={tags}&limit={limit}";

    fn new(builder: Self::Builder) -> Self {
        GelbooruClient {
            options: builder.options.into(),
        }
    }

    fn options(&'_ self) -> &'_ BooruClientOptions {
        &self.options
    }
}

#[derive(Default)]
pub struct GelbooruClientBuilder {
    options: BooruClientBuilderOptions,
}

impl BooruClientBuilder for GelbooruClientBuilder {
    type Client = GelbooruClient;
    type Rating = GelbooruRating;
    type Order = GelbooruSort;

    const BASE_URL: &'static str = "https://gelbooru.com";

    fn new() -> Self {
        GelbooruClientBuilder {
            options: BooruClientBuilderOptions::with_url(Self::BASE_URL),
        }
    }
    fn build(self) -> Self::Client
    where
        Self: Sized,
    {
        Self::Client::new(self)
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
        Self: Sized,
    {
        self.tag(format!("sort:{}", order))
    }

    fn with_inner_options<F>(mut self, func: F) -> Self
    where
        F: FnOnce(BooruClientBuilderOptions) -> BooruClientBuilderOptions,
    {
        self.options = func(self.options);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gelbooru() {
        let builder = GelbooruClientBuilder::new();
    }
}
