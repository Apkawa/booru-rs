use self::model::{GelbooruPost, GelbooruRating, GelbooruResponse, GelbooruSort};
use crate::client::generic::{BooruClient, BooruClientOptions, BooruOptionBuilder};

#[cfg(feature = "gelbooru")]
pub mod model;
mod format;

/// Client that sends requests to the Gelbooru >=v0.2.5 API to retrieve the data.
pub struct GelbooruClient {
    options: BooruClientOptions,
}

impl BooruClient for GelbooruClient {
    type PostModel = GelbooruPost;
    type PostResponse = GelbooruResponse;
    type PostListResponse = GelbooruResponse;
    type Rating = GelbooruRating;
    type Order = GelbooruSort;
    const BASE_URL: &'static str = "https://gelbooru.com";
    const PATH_POST_BY_ID: &'static str = "index.php?page=dapi&s=post&q=index&json=1&id={id}";
    const PATH_POST: &'static str =
        "index.php?page=dapi&s=post&q=index&json=1&pid={page}&tags={tags}&limit={limit}";

    fn with_options(options: BooruClientOptions) -> Self {
        GelbooruClient {
            options: options.into(),
        }
    }

    fn options(&'_ self) -> &'_ BooruClientOptions {
        &self.options
    }

    fn url_posts(&self) -> String {
        let options = self.options();
        let page = options.page;
        let mut tag_string = options.tags.join(" ");

        // https://gelbooru.com/index.php?page=help&topic=cheatsheet
        if let Some(order) = options.order.as_ref() {
            tag_string.push_str(format!(" sort:{order}").as_str());
        }
        if let Some(random) = options.random.as_ref() {
            if *random {
                tag_string.push_str(format!(" sort:random").as_str());
            }
        }
        if let Some(rating) = options.rating.as_ref() {
            tag_string.push_str(format!(" rating:{rating}").as_str());
        }
        let tag_string = form_urlencoded::byte_serialize(tag_string.as_bytes());

        [&self.base_url(), Self::PATH_POST]
            .join("/")
            .replace("{page}", &page.to_string())
            .replace("{limit}", &self.options().limit.to_string())
            .replace("{tags}", &tag_string.collect::<String>())
    }
}

impl BooruOptionBuilder for GelbooruClient {
    fn with_inner_options<F>(mut self, func: F) -> Self
    where
        F: FnOnce(BooruClientOptions) -> BooruClientOptions,
    {
        self.options = func(self.options);
        self
    }
}

