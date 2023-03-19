use std::str::FromStr;


use crate::manager::builder::EngineBooruBuilder;

pub mod builder;


#[derive(Debug, Clone, PartialEq)]
pub enum Engine {
    Danbooru,
    DanbooruV1,
    Gelbooru,
    GelbooruV02,
    Moebooru,
    Philomena,
    Zerochan,
    E621ng,
}

impl Engine {
    pub fn builder(&self) -> EngineBooruBuilder {
        EngineBooruBuilder::new(self.to_owned())
    }
}

impl FromStr for Engine {
    type Err = ();

    ///
    /// ```
    /// use std::str::FromStr;
    /// use booru_rs::manager::Engine;
    /// assert_eq!(Engine::from_str("danbooru").unwrap(), Engine::Danbooru);
    /// assert_eq!(Engine::from_str("DanbOOru").unwrap(), Engine::Danbooru);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Engine::*;
        let engine = match s.to_lowercase().as_str() {
            "danbooru" => Danbooru,
            "danbooruv1" => DanbooruV1,
            "gelbooru" => Gelbooru,
            "gelbooruv2" => GelbooruV02,
            "moebooru" => Moebooru,
            "philomena" => Philomena,
            "zerochan" => Zerochan,
            "e621ng" => E621ng,
            _ => return Err(())
        };
        Ok(engine)
    }
}

impl TryFrom<&str> for Engine {
    type Error = ();

    ///
    /// ```
    /// use booru_rs::manager::Engine;
    /// let e: Engine = "danbooru".try_into().unwrap();
    /// assert_eq!(e, Engine::Danbooru)
    /// ```
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Engine::from_str(value)
    }
}

impl TryFrom<&String> for Engine {
    type Error = ();

    ///
    /// ```
    /// use booru_rs::manager::Engine;
    /// let s: &String = &"danbooru".to_string();
    /// let e: Engine = s.try_into().unwrap();
    /// assert_eq!(e, Engine::Danbooru)
    /// ```
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Engine::from_str(value.as_str())
    }
}


#[cfg(test)]
mod tests {
    use crate::client::generic::BooruOptionBuilder;
    use super::*;

    #[test]
    fn test_danbooru() {
        let post = Engine::Danbooru.builder()
            .default_url("https://testbooru.donmai.us")
            .get_by_id(9423)
            .unwrap();
        assert_eq!(post.id().to_string(), "9423".to_string())
    }
}