#[cfg(feature = "gelbooru")]
mod gelbooru {
    use booru_rs::client::danbooru_v1::{DanbooruClientV1, DanbooruPostV1};
    use booru_rs::client::generic::{BooruClient, BooruClientBuilder};

    use crate::helpers::{load_json_fixture, proxy};

    #[test]
    fn get_posts_with_tag() {
        let posts = DanbooruClientV1::builder()
            .tag("elbow_gloves")
            .build()
            .get()
            .unwrap();

        assert!(!posts.is_empty());
    }

    #[test]
    fn get_post_by_id() {
        let post = DanbooruClientV1::builder()
            .build()
            .get_by_id(650866);

        assert_eq!("5988fc36df3c784771419dd8a068b3a0", post.unwrap().md5);
    }

    #[test]
    fn posts_deserialize_json() {
        let json: Vec<DanbooruPostV1> = serde_json::from_str(load_json_fixture("danbooru_v1/posts").as_str()).unwrap();
        assert_eq!(json.len(), 10);
    }

    #[test]
    fn post_deserialize_json() {
        let json: Vec<DanbooruPostV1> = serde_json::from_str(load_json_fixture("danbooru_v1/post_id").as_str()).unwrap();
        let model: DanbooruPostV1 = json.into();
        assert_eq!(model.id, 650866);
    }
}


