pub(crate) mod timestamp_to_rfc_3339 {
    use serde::{self, Deserialize, Serializer, Deserializer};
    use time::format_description::well_known::Rfc3339;
    use time::OffsetDateTime;


    pub fn serialize<S>(
        _s: &String,
        _serializer: S,
    ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        // We not need serialize back
        unimplemented!()
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<String, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = i64::deserialize(deserializer)?;
        let parsed = OffsetDateTime::from_unix_timestamp(s).unwrap();

        Ok(format!("{}", parsed.format(&Rfc3339).unwrap()))
    }
}
