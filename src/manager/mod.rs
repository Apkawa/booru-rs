use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

use crate::manager::builder::EngineBooruBuilder;

pub mod builder;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Engine {
    #[cfg(feature = "danbooru")]
    Danbooru,
    #[cfg(feature = "danbooru_v1")]
    DanbooruV1,
    #[cfg(feature = "gelbooru")]
    Gelbooru,
    #[cfg(feature = "gelbooru_v02")]
    GelbooruV02,
    #[cfg(feature = "moebooru")]
    Moebooru,
    #[cfg(feature = "philomena")]
    Philomena,
    #[cfg(feature = "zerochan")]
    Zerochan,
    #[cfg(feature = "e621ng")]
    E621ng,
    // Danbooru,
    // DanbooruV1,
    // Gelbooru,
    // GelbooruV02,
    // Moebooru,
    // Philomena,
    // Zerochan,
    // E621ng,
}

impl Engine {
    pub fn builder(&self) -> EngineBooruBuilder {
        EngineBooruBuilder::new(self.to_owned())
    }
}

impl<'de> Deserialize<'de> for Engine {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Engine::try_from(&s).map_err(|_| D::Error::custom("Invalid engine!"))
    }
}

impl ToString for Engine {
    fn to_string(&self) -> String {
        format!("{:?}", &self).to_lowercase()
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
    /// assert_eq!(Engine::from_str("gelbooru_v0.2").unwrap(), Engine::GelbooruV02);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Engine::*;
        let s = s.to_lowercase();
        let s = s.replace("_", "").replace("-", "").replace(".", "");
        let engine = match s.as_str() {
            #[cfg(feature = "danbooru")]
            "danbooru" => Danbooru,
            #[cfg(feature = "danbooru_v1")]
            "danbooruv1" => DanbooruV1,
            #[cfg(feature = "gelbooru")]
            "gelbooru" => Gelbooru,
            #[cfg(feature = "gelbooru_v02")]
            "gelbooruv02" | "gelbooruv2" => GelbooruV02,
            #[cfg(feature = "gelbooru")]
            "moebooru" => Moebooru,
            #[cfg(feature = "philomena")]
            "philomena" => Philomena,
            #[cfg(feature = "zerochan")]
            "zerochan" => Zerochan,
            #[cfg(feature = "e621ng")]
            "e621ng" => E621ng,
            _ => return Err(()),
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
    use super::*;
    use crate::client::generic::BooruOptionBuilder;

    #[test]
    fn test_danbooru() {
        let post = Engine::Danbooru
            .builder()
            .url("https://testbooru.donmai.us")
            .get_by_id(9423)
            .unwrap();
        assert_eq!(post.id().to_string(), "9423".to_string())
    }
}
