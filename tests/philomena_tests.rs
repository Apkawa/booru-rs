
mod philomena {
    use booru_rs::client::generic::{BooruClient, BooruClientBuilder};
    use booru_rs::client::philomena::{PhilomenaClient, PhilomenaDetailResponse, PhilomenaListResponse, PhilomenaPost};
    use crate::helpers::{load_json_fixture, proxy};

    #[test]
    fn get_posts_with_tag() {
        let posts = PhilomenaClient::builder()
            .tag("safe")
            .build()
            .get()
            .unwrap();

        assert!(!posts.is_empty());
    }

    #[test]
    fn get_post_by_id() {
        let post = PhilomenaClient::builder()
            .build()
            .get_by_id(1);

        assert_eq!(post.unwrap().id, 1);
    }

    #[test]
    fn posts_deserialize_json() {
        let json: PhilomenaListResponse = serde_json::from_str(load_json_fixture("philomena/posts").as_str()).unwrap();
        let model: Vec<PhilomenaPost> = json.into();
        assert_eq!(model.len(), 10);
    }

    #[test]
    fn post_deserialize_json() {
        let json: PhilomenaDetailResponse = serde_json::from_str(load_json_fixture("philomena/post_id").as_str()).unwrap();
        let model: PhilomenaPost = json.into();
        assert_eq!(model.id, 1);
    }
}