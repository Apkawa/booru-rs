pub(crate) mod gelbooru_format_to_rfc_3339 {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use time::format_description::well_known::Rfc3339;
    use time::macros::format_description;
    use time::OffsetDateTime;

    pub fn serialize<S>(_s: &str, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        unimplemented!()
    }
    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let gelbooru_format = format_description!(
            "[weekday repr:short] [month repr:short] [day] \
    [hour]:[minute]:[second] [offset_hour][offset_minute] [year]"
        );
        let s = String::deserialize(deserializer)?;
        let parsed = OffsetDateTime::parse(&s, &gelbooru_format).unwrap();

        Ok(parsed.format(&Rfc3339).unwrap())
    }
}
