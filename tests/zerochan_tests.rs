#[cfg(feature = "zerochan")]
#[cfg(test)]
mod zerochan {
    use crate::helpers::{load_json_fixture, proxy};
    use booru_rs::client::generic::{BooruClient, BooruOptionBuilder, BooruPostModel};
    use booru_rs::client::zerochan::{ZerochanClient, ZerochanListResponse, ZerochanPost};

    #[test]
    fn get_posts_with_tag() {
        let posts = ZerochanClient::new()
            .proxy(proxy())
            .limit(100)
            .tag("Shampoo (Ranma ½)")
            .get()
            .unwrap();

        assert!(!posts.is_empty());
        // dbg!(posts);
    }

    #[test]
    fn get_post_by_id() {
        let post = ZerochanClient::new().proxy(proxy()).get_by_id(3914229);

        assert_eq!(post.unwrap().id, 3914229);
    }

    #[test]
    fn posts_deserialize_json() {
        let json: ZerochanListResponse =
            serde_json::from_str(load_json_fixture("zerochan/posts").as_str()).unwrap();
        let model: Vec<ZerochanPost> = json.into();
        assert_eq!(model.len(), 24);
    }

    #[test]
    fn post_deserialize_json() {
        let json: ZerochanPost =
            serde_json::from_str(load_json_fixture("zerochan/post_id").as_str()).unwrap();
        let model: ZerochanPost = json.into();
        assert_eq!(model.id, 3914235);
    }

    #[test]
    fn post_booru_model_trait() {
        let json: ZerochanPost =
            serde_json::from_str(load_json_fixture("zerochan/post_id").as_str()).unwrap();
        let model: ZerochanPost = json.into();
        assert_eq!(model.id().to_string(), model.id.to_string());
        assert_eq!(
            model.hash().as_ref().unwrap().to_string().as_str(),
            model.hash.as_ref().unwrap()
        );
        let images = model.images();
        assert_eq!(
            images.original.as_ref().unwrap().url.to_string(),
            model.full
        )
    }
}
