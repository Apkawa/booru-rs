#[cfg(feature = "gelbooru_v02")]
#[cfg(test)]
mod gelbooru_v02 {
    use booru_rs::client::gelbooru_v0_2::model::GelbooruPostV0_2;
    use booru_rs::client::gelbooru_v0_2::GelbooruClientV0_2;
    use booru_rs::client::generic::{BooruClient, BooruOptionBuilder, BooruPostModel};

    use crate::helpers::{load_json_fixture, proxy};

    #[test]
    fn get_posts_with_tag() {
        let posts = GelbooruClientV0_2::new()
            .proxy(proxy())
            .tag("kafuu_chino")
            .get()
            .unwrap();

        assert!(!posts.is_empty());
    }

    #[test]
    fn get_post_by_id() {
        let post = GelbooruClientV0_2::new()
            .proxy(proxy())
            .get_by_id(4296500);

        assert_eq!("c0bcc488ee06ff3e1708287b00ba1661", post.unwrap().hash);
    }

    #[test]
    fn posts_deserialize_json() {
        let json: Vec<GelbooruPostV0_2> =
            serde_json::from_str(load_json_fixture("gelbooru_v0.2/posts").as_str()).unwrap();
        assert_eq!(json.len(), 2);
    }

    #[test]
    fn post_deserialize_json() {
        let json: Vec<GelbooruPostV0_2> =
            serde_json::from_str(load_json_fixture("gelbooru_v0.2/post_id").as_str()).unwrap();
        let model: GelbooruPostV0_2 = json.into();
        assert_eq!(model.id, 4296456);
    }

    #[test]
    fn post_booru_model_trait() {
        let json: Vec<GelbooruPostV0_2> =
            serde_json::from_str(load_json_fixture("gelbooru_v0.2/post_id").as_str()).unwrap();
        let model: GelbooruPostV0_2 = json.into();
        assert_eq!(model.id().to_string(), model.id.to_string());
        assert_eq!(
            model.hash().as_ref().unwrap().to_string().as_str(),
            model.hash.as_str()
        );
        let images = model.images();
        assert_eq!(
            images.original.as_ref().unwrap().url.to_string(),
            model.image
        )
    }
}
