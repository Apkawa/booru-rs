#[cfg(feature = "zerochan")]
#[cfg(test)]
mod zerochan {
    use std::fs::read_to_string;
    use booru_rs::client::generic::{BooruClient, BooruClientBuilder};
    use booru_rs::client::zerochan::{ZerochanClient, ZerochanListResponse, ZerochanPost};
    use crate::helpers::{load_json_fixture, proxy};

    #[test]
    fn get_posts_with_tag() {
        let posts = ZerochanClient::builder()
            .proxy(proxy())
            .limit(100)
            .tag("Shampoo (Ranma ½)")
            .build()
            .get()
            .unwrap();

        assert!(!posts.is_empty());
        // dbg!(posts);
    }

    #[test]
    fn get_post_by_id() {
        let post = ZerochanClient::builder()
            .proxy(proxy())
            .build()
            .get_by_id(3914229);

        assert_eq!(post.unwrap().id, 3914229);
    }

    #[test]
    fn post_deserialize_json() {
        let json: ZerochanPost = serde_json::from_str(load_json_fixture("zerochan/post_id").as_str()).unwrap();
        let model: ZerochanPost = json.into();
        assert_eq!(model.id, 3914235);
    }
}