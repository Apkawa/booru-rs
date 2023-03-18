#[cfg(feature = "moebooru")]
#[cfg(test)]
mod myimouto {
    use booru_rs::client::generic::{BooruClient, BooruClientBuilder};
    use booru_rs::client::moebooru::model::MoebooruPost;
    use booru_rs::client::moebooru::MoebooruClient;

    use crate::helpers::{load_json_fixture, proxy};

    #[test]
    fn get_posts_with_tag() {
        let posts = MoebooruClient::builder()
            .default_url("https://lolibooru.moe")
            .proxy(proxy())
            .tag("kafuu_chino")
            .build()
            .get()
            .unwrap();

        assert!(!posts.is_empty());
    }

    #[test]
    fn get_post_by_id() {
        let post = MoebooruClient::builder()
            .default_url("https://lolibooru.moe")
            .proxy(proxy())
            .build()
            .get_by_id(543968);

        assert_eq!("64bb4cbb5e9ae271b1ad76b1af112c53", post.unwrap().md5);
    }

    #[test]
    fn posts_deserialize_json() {
        let json: Vec<MoebooruPost> = serde_json::from_str(load_json_fixture("myimouto/posts").as_str()).unwrap();
        assert_eq!(json.len(), 10);
    }

    #[test]
    fn post_deserialize_json() {
        let json: Vec<MoebooruPost> = serde_json::from_str(load_json_fixture("myimouto/post_id").as_str()).unwrap();
        let model: MoebooruPost = json.into();
        assert_eq!(model.id, 543968);
    }
}