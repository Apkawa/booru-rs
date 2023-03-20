#[cfg(feature = "moebooru")]
#[cfg(test)]
mod myimouto {
    use booru_rs::client::generic::{BooruClient, BooruOptionBuilder, BooruPostModel};
    use booru_rs::client::moebooru::model::MoebooruPost;
    use booru_rs::client::moebooru::MoebooruClient;

    use crate::helpers::{load_json_fixture, proxy};

    #[test]
    fn get_posts_with_tag() {
        let posts = MoebooruClient::new()
            .url("https://lolibooru.moe")
            .proxy(proxy())
            .tag("kafuu_chino")
            .get()
            .unwrap();

        assert!(!posts.is_empty());
    }

    #[test]
    fn get_post_by_id() {
        let post = MoebooruClient::new()
            .url("https://lolibooru.moe")
            .proxy(proxy())
            .get_by_id(543968);

        assert_eq!("64bb4cbb5e9ae271b1ad76b1af112c53", post.unwrap().md5);
    }

    #[test]
    fn posts_deserialize_json() {
        let json: Vec<MoebooruPost> =
            serde_json::from_str(load_json_fixture("myimouto/posts").as_str()).unwrap();
        assert_eq!(json.len(), 10);
    }

    #[test]
    fn post_deserialize_json() {
        let json: Vec<MoebooruPost> =
            serde_json::from_str(load_json_fixture("myimouto/post_id").as_str()).unwrap();
        let model: MoebooruPost = json.into();
        assert_eq!(model.id, 543968);
    }

    #[test]
    fn post_booru_model_trait() {
        let json: Vec<MoebooruPost> =
            serde_json::from_str(load_json_fixture("myimouto/post_id").as_str()).unwrap();
        let model: MoebooruPost = json.into();
        assert_eq!(model.id().to_string(), model.id.to_string());
        assert_eq!(model.hash().as_ref().unwrap().to_string(), model.md5);
        let images = model.images();
        assert_eq!(
            images.original.as_ref().unwrap().url.to_string(),
            model.file_url
        )
    }
}
