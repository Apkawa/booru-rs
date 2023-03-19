#[cfg(feature = "philomena")]
#[cfg(test)]
mod philomena {
    use crate::helpers::load_json_fixture;
    use booru_rs::client::generic::{BooruClient, BooruOptionBuilder, BooruPostModel};
    use booru_rs::client::philomena::{
        PhilomenaClient, PhilomenaDetailResponse, PhilomenaListResponse, PhilomenaPost,
    };

    #[test]
    fn get_posts_with_tag() {
        let posts = PhilomenaClient::new().tag("safe").get().unwrap();

        assert!(!posts.is_empty());
    }

    #[test]
    fn get_post_by_id() {
        let post = PhilomenaClient::new().get_by_id(1);

        assert_eq!(post.unwrap().id, 1);
    }

    #[test]
    fn posts_deserialize_json() {
        let json: PhilomenaListResponse =
            serde_json::from_str(load_json_fixture("philomena/posts").as_str()).unwrap();
        let model: Vec<PhilomenaPost> = json.into();
        assert_eq!(model.len(), 10);
    }

    #[test]
    fn post_deserialize_json() {
        let json: PhilomenaDetailResponse =
            serde_json::from_str(load_json_fixture("philomena/post_id").as_str()).unwrap();
        let model: PhilomenaPost = json.into();
        assert_eq!(model.id, 1);
    }

    #[test]
    fn post_booru_model_trait() {
        let json: PhilomenaDetailResponse =
            serde_json::from_str(load_json_fixture("philomena/post_id").as_str()).unwrap();
        let model: PhilomenaPost = json.into();
        assert_eq!(model.id().to_string(), model.id.to_string());
        assert_eq!(
            model.hash().as_ref().unwrap().to_string(),
            model.sha512_hash
        );
        let images = model.images();
        assert_eq!(
            images.original.as_ref().unwrap().url.to_string(),
            model.view_url
        )
    }
}
