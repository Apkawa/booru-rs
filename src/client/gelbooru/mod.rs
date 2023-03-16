use crate::client::generic::{BooruClient, BooruClientBuilder, BooruClientBuilderOptions, BooruClientOptions};
use crate::model::gelbooru::{GelbooruPost, GelbooruRating, GelbooruResponse, GelbooruSort};


/// Client that sends requests to the Gelbooru API to retrieve the data.
pub struct GelbooruClient {
    options: BooruClientOptions,
}

impl BooruClient<'_> for GelbooruClient {
    type Builder = GelbooruClientBuilder;
    type PostModel = GelbooruPost;
    type PostListModel = GelbooruResponse;
    const PATH_POST_BY_ID: &'static str = "index.php?page=dapi&s=post&q=index&json=1&id={id}";
    const PATH_POST: &'static str = "index.php?page=dapi&s=post&q=index&json=1";

    fn new(options: BooruClientBuilderOptions) -> Self {
        GelbooruClient { options: options.into() }
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
            options: BooruClientBuilderOptions::with_url(Self::BASE_URL)
        }
    }

    fn build(self) -> Self::Client
        where Self: Sized {
        Self::Client::new(self.options)
    }

    fn random(self, random: bool) -> Self {
        if random {
            self.tag("sort:random".to_string())
        } else {
            self
        }
    }

    fn with_inner_options<F>(mut self, func: F) -> Self
        where F: FnOnce(BooruClientBuilderOptions) -> BooruClientBuilderOptions {
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