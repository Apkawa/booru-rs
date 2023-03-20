use std::fmt::Display;

#[cfg(feature = "danbooru")]
use crate::client::danbooru::DanbooruClient;
#[cfg(feature = "danbooru_v1")]
use crate::client::danbooru_v1::DanbooruV1Client;
#[cfg(feature = "e621ng")]
use crate::client::e621ng::E621ngClient;
#[cfg(feature = "gelbooru")]
use crate::client::gelbooru::GelbooruClient;
#[cfg(feature = "gelbooru_v02")]
use crate::client::gelbooru_v0_2::GelbooruV02Client;
use crate::client::generic::{BooruClient, BooruClientOptions, BooruOptionBuilder, BooruPostModel};
#[cfg(feature = "moebooru")]
use crate::client::moebooru::MoebooruClient;
#[cfg(feature = "philomena")]
use crate::client::philomena::PhilomenaClient;
#[cfg(feature = "zerochan")]
use crate::client::zerochan::ZerochanClient;
use crate::manager::Engine;

#[derive(Debug)]
pub struct EngineBooruBuilder {
    engine: Engine,
    options: BooruClientOptions,
}

impl EngineBooruBuilder {
    pub fn new(engine: Engine) -> Self {
        EngineBooruBuilder {
            engine,
            options: Default::default(),
        }
    }
    pub fn client(&self) -> reqwest::blocking::Client {
        self.options.client()
    }

    pub fn base_url(&self) -> String {
        use Engine::*;
        match self.engine {
            #[cfg(feature = "danbooru")]
            Danbooru => DanbooruClient::with_options(self.options.clone()).base_url().to_string(),
            #[cfg(feature = "danbooru_v1")]
            DanbooruV1 => DanbooruV1Client::with_options(self.options.clone()).base_url().to_string(),
            #[cfg(feature = "gelbooru")]
            Gelbooru => GelbooruClient::with_options(self.options.clone()).base_url().to_string(),
            #[cfg(feature = "gelbooru_v02")]
            GelbooruV02 => GelbooruV02Client::with_options(self.options.clone()).base_url().to_string(),
            #[cfg(feature = "moebooru")]
            Moebooru => MoebooruClient::with_options(self.options.clone()).base_url().to_string(),
            #[cfg(feature = "philomena")]
            Philomena => PhilomenaClient::with_options(self.options.clone()).base_url().to_string(),
            #[cfg(feature = "zerochan")]
            Zerochan => ZerochanClient::with_options(self.options.clone()).base_url().to_string(),
            #[cfg(feature = "e621ng")]
            E621ng => E621ngClient::with_options(self.options.clone()).base_url().to_string(),
        }
    }

    /// Directly get a post by its unique Id
    pub fn get_by_id<I: Display>(&self, id: I) -> Result<Box<dyn BooruPostModel>, reqwest::Error> {
        use Engine::*;
        let res: Box<dyn BooruPostModel> = match self.engine {
            #[cfg(feature = "danbooru")]
            Danbooru => Box::new(DanbooruClient::with_options(self.options.clone()).get_by_id(id)?),
            #[cfg(feature = "danbooru_v1")]
            DanbooruV1 => {
                Box::new(DanbooruV1Client::with_options(self.options.clone()).get_by_id(id)?)
            }
            #[cfg(feature = "gelbooru")]
            Gelbooru => Box::new(GelbooruClient::with_options(self.options.clone()).get_by_id(id)?),
            #[cfg(feature = "gelbooru_v02")]
            GelbooruV02 => {
                Box::new(GelbooruV02Client::with_options(self.options.clone()).get_by_id(id)?)
            }
            #[cfg(feature = "moebooru")]
            Moebooru => Box::new(MoebooruClient::with_options(self.options.clone()).get_by_id(id)?),
            #[cfg(feature = "philomena")]
            Philomena => {
                Box::new(PhilomenaClient::with_options(self.options.clone()).get_by_id(id)?)
            }
            #[cfg(feature = "zerochan")]
            Zerochan => Box::new(ZerochanClient::with_options(self.options.clone()).get_by_id(id)?),
            #[cfg(feature = "e621ng")]
            E621ng => Box::new(E621ngClient::with_options(self.options.clone()).get_by_id(id)?),
        };
        Ok(res)
    }

    /// Get first page
    pub fn get(&self) -> Result<Vec<Box<dyn BooruPostModel>>, reqwest::Error> {
        use Engine::*;

        let res: Vec<Box<dyn BooruPostModel>> = match self.engine {
            #[cfg(feature = "danbooru")]
            Danbooru => DanbooruClient::with_options(self.options.clone())
                .get()?
                .into_iter()
                .map(|s| Box::new(s) as Box<dyn BooruPostModel>)
                .collect(),
            #[cfg(feature = "danbooru_v1")]
            DanbooruV1 => DanbooruV1Client::with_options(self.options.clone())
                .get()?
                .into_iter()
                .map(|s| Box::new(s) as Box<dyn BooruPostModel>)
                .collect(),
            #[cfg(feature = "gelbooru")]
            Gelbooru => GelbooruClient::with_options(self.options.clone())
                .get()?
                .into_iter()
                .map(|s| Box::new(s) as Box<dyn BooruPostModel>)
                .collect(),
            #[cfg(feature = "gelbooru_v02")]
            GelbooruV02 => GelbooruV02Client::with_options(self.options.clone())
                .get()?
                .into_iter()
                .map(|s| Box::new(s) as Box<dyn BooruPostModel>)
                .collect(),

            #[cfg(feature = "moebooru")]
            Moebooru => MoebooruClient::with_options(self.options.clone())
                .get()?
                .into_iter()
                .map(|s| Box::new(s) as Box<dyn BooruPostModel>)
                .collect(),
            #[cfg(feature = "philomena")]
            Philomena => PhilomenaClient::with_options(self.options.clone())
                .get()?
                .into_iter()
                .map(|s| Box::new(s) as Box<dyn BooruPostModel>)
                .collect(),
            #[cfg(feature = "zerochan")]
            Zerochan => ZerochanClient::with_options(self.options.clone())
                .get()?
                .into_iter()
                .map(|s| Box::new(s) as Box<dyn BooruPostModel>)
                .collect(),
            #[cfg(feature = "e621ng")]
            E621ng => E621ngClient::with_options(self.options.clone())
                .get()?
                .into_iter()
                .map(|s| Box::new(s) as Box<dyn BooruPostModel>)
                .collect(),
        };
        Ok(res)
    }
}

impl BooruOptionBuilder for EngineBooruBuilder {
    fn with_inner_options<F>(mut self, func: F) -> Self
        where
            F: FnOnce(BooruClientOptions) -> BooruClientOptions,
    {
        self.options = func(self.options);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_danbooru_post_list() {
        let posts = EngineBooruBuilder::new(Engine::Danbooru)
            .url("https://testbooru.donmai.us")
            .limit(10)
            .tag("1girl")
            .get()
            .unwrap();

        assert_eq!(posts.len(), 10)
    }

    #[test]
    fn test_danbooru_post() {
        let post = EngineBooruBuilder::new(Engine::Danbooru)
            .url("https://testbooru.donmai.us")
            .get_by_id(9423)
            .unwrap();

        assert_eq!(post.id().to_string(), "9423".to_string())
    }
}
