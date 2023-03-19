#[cfg(feature = "moebooru")]
#[cfg(test)]
mod moebooru {
    use booru_rs::client::generic::{BooruClient, BooruOptionBuilder, BooruPostModel};
    use booru_rs::client::moebooru::model::MoebooruPost;
    use booru_rs::client::moebooru::MoebooruClient;

    use crate::helpers::{load_json_fixture, proxy};

    #[test]
    fn get_posts_with_tag() {
        let posts = MoebooruClient::new()
            .proxy(proxy())
            .tag("kafuu_chino")
            .get()
            .unwrap();

        assert!(!posts.is_empty());
    }

    #[test]
    fn get_post_by_id() {
        let post = MoebooruClient::new()
            .proxy(proxy())
            .get_by_id(354317);

        assert_eq!("3f2eee84abba3e65072d74bc945467b9", post.unwrap().md5);
    }

    #[test]
    fn posts_deserialize_json() {
        let json: Vec<MoebooruPost> =
            serde_json::from_str(load_json_fixture("moebooru/posts").as_str()).unwrap();
        assert_eq!(json.len(), 10);
    }

    #[test]
    fn post_deserialize_json() {
        let json: Vec<MoebooruPost> =
            serde_json::from_str(load_json_fixture("moebooru/post_id").as_str()).unwrap();
        let model: MoebooruPost = json.into();
        assert_eq!(model.id, 354323);
    }

    #[test]
    fn post_booru_model_trait() {
        let json: Vec<MoebooruPost> =
            serde_json::from_str(load_json_fixture("moebooru/post_id").as_str()).unwrap();
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
