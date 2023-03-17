#[cfg(feature = "gelbooru")]
mod gelbooru {
    use booru_rs::client::gelbooru_v0_2::GelbooruClientV0_2;
    use booru_rs::client::gelbooru_v0_2::model::GelbooruPostV0_2;
    use booru_rs::client::generic::{BooruClient, BooruClientBuilder};

    use crate::helpers::{load_json_fixture, proxy};

    #[test]
    fn get_posts_with_tag() {
        let posts = GelbooruClientV0_2::builder()
            .proxy(proxy())
            .tag("kafuu_chino")
            .build()
            .get()
            .unwrap();

        assert!(!posts.is_empty());
    }

    #[test]
    fn get_post_by_id() {
        let post = GelbooruClientV0_2::builder()
            .proxy(proxy())
            .build()
            .get_by_id(4296500);

        assert_eq!("c0bcc488ee06ff3e1708287b00ba1661", post.unwrap().hash);
    }

    #[test]
    fn posts_deserialize_json() {
        let json: Vec<GelbooruPostV0_2> = serde_json::from_str(load_json_fixture("gelbooru_v0.2/posts").as_str()).unwrap();
        assert_eq!(json.len(), 2);
    }

    #[test]
    fn post_deserialize_json() {
        let json: Vec<GelbooruPostV0_2> = serde_json::from_str(load_json_fixture("gelbooru_v0.2/post_id").as_str()).unwrap();
        let model: GelbooruPostV0_2 = json.into();
        assert_eq!(model.id, 4296456);
    }
}


