use std::fmt::Display;

use crate::client::danbooru::DanbooruClient;
use crate::client::danbooru_v1::DanbooruV1Client;
use crate::client::e621ng::E621ngClient;
use crate::client::gelbooru::GelbooruClient;
use crate::client::gelbooru_v0_2::GelbooruV02Client;
use crate::client::generic::{BooruClient, BooruClientOptions, BooruOptionBuilder, BooruPostModel};
use crate::client::moebooru::MoebooruClient;
use crate::client::philomena::PhilomenaClient;
use crate::client::zerochan::ZerochanClient;
use crate::manager::Engine;

#[derive(Debug)]
pub struct EngineBooruBuilder {
    engine: Engine,
    options: BooruClientOptions,
}

impl EngineBooruBuilder {
    pub fn new(engine: Engine) -> Self {
        EngineBooruBuilder { engine, options: Default::default() }
    }
    /// Directly get a post by its unique Id
    pub fn get_by_id<'b, I: Display>(&self, id: I) -> Result<Box<dyn BooruPostModel>, reqwest::Error> {
        use Engine::*;
        let res: Box<dyn BooruPostModel> = match self.engine {
            Danbooru => Box::new(DanbooruClient::with_options(self.options.clone()).get_by_id(id)?),
            DanbooruV1 => Box::new(DanbooruV1Client::with_options(self.options.clone()).get_by_id(id)?),
            Gelbooru => Box::new(GelbooruClient::with_options(self.options.clone()).get_by_id(id)?),
            GelbooruV02 => Box::new(GelbooruV02Client::with_options(self.options.clone()).get_by_id(id)?),
            Moebooru => Box::new(MoebooruClient::with_options(self.options.clone()).get_by_id(id)?),
            Philomena => Box::new(PhilomenaClient::with_options(self.options.clone()).get_by_id(id)?),
            Zerochan => Box::new(ZerochanClient::with_options(self.options.clone()).get_by_id(id)?),
            E621ng => Box::new(E621ngClient::with_options(self.options.clone()).get_by_id(id)?)
        };
        Ok(res)
    }

    /// Get first page
    pub fn get(&self) -> Result<Vec<Box<dyn BooruPostModel>>, reqwest::Error> {
        use Engine::*;

        let res: Vec<Box<dyn BooruPostModel>> = match self.engine {
            Danbooru => DanbooruClient::with_options(self.options.clone())
                .get()?
                .into_iter()
                .map(|s| Box::new(s) as Box<dyn BooruPostModel>)
                .collect(),
            DanbooruV1 => DanbooruV1Client::with_options(self.options.clone())
                .get()?
                .into_iter()
                .map(|s| Box::new(s) as Box<dyn BooruPostModel>)
                .collect(),
            Gelbooru => GelbooruClient::with_options(self.options.clone())
                .get()?
                .into_iter()
                .map(|s| Box::new(s) as Box<dyn BooruPostModel>)
                .collect(),
            GelbooruV02 => GelbooruV02Client::with_options(self.options.clone())
                .get()?
                .into_iter()
                .map(|s| Box::new(s) as Box<dyn BooruPostModel>)
                .collect(),

            Moebooru => MoebooruClient::with_options(self.options.clone())
                .get()?
                .into_iter()
                .map(|s| Box::new(s) as Box<dyn BooruPostModel>)
                .collect(),
            Philomena => PhilomenaClient::with_options(self.options.clone())
                .get()?
                .into_iter()
                .map(|s| Box::new(s) as Box<dyn BooruPostModel>)
                .collect(),
            Zerochan => ZerochanClient::with_options(self.options.clone())
                .get()?
                .into_iter()
                .map(|s| Box::new(s) as Box<dyn BooruPostModel>)
                .collect(),
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
            .default_url("https://testbooru.donmai.us")
            .limit(10)
            .tag("1girl")
            .get().unwrap();

        assert_eq!(posts.len(), 10)
    }

    #[test]
    fn test_danbooru_post() {
        let post = EngineBooruBuilder::new(Engine::Danbooru)
            .default_url("https://testbooru.donmai.us")
            .get_by_id(9423).unwrap();

        assert_eq!(post.id().to_string(), "9423".to_string())
    }
}