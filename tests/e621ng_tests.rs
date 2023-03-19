#[cfg(feature = "e621ng")]
#[cfg(test)]
mod e621ng {
    use crate::helpers::{load_json_fixture, proxy};
    use booru_rs::client::e621ng::model::{E621ngDetailResponse, E621ngListResponse};
    use booru_rs::client::e621ng::{E621ngClient, E621ngPost};
    use booru_rs::client::generic::{BooruClient, BooruClientBuilder, BooruPostModel};

    #[test]
    fn get_posts_with_tag() {
        let posts = E621ngClient::builder()
            .proxy(proxy())
            .limit(100)
            .tag("domestic_cat")
            .build()
            .get()
            .unwrap();

        assert!(!posts.is_empty());
        // dbg!(posts);
    }

    #[test]
    fn get_post_by_id() {
        let post = E621ngClient::builder()
            .proxy(proxy())
            .build()
            .get_by_id(3937132);

        assert_eq!(post.unwrap().id, 3937132);
    }

    #[test]
    fn posts_deserialize_json() {
        let json: E621ngListResponse =
            serde_json::from_str(load_json_fixture("e621ng/posts").as_str()).unwrap();
        let model: Vec<E621ngPost> = json.into();
        assert_eq!(model.len(), 10);
    }

    #[test]
    fn post_deserialize_json() {
        let json: E621ngDetailResponse =
            serde_json::from_str(load_json_fixture("e621ng/post_id").as_str()).unwrap();
        let model: E621ngPost = json.into();
        assert_eq!(model.id, 3937132);
    }

    #[test]
    fn post_booru_model_trait() {
        let json: E621ngDetailResponse =
            serde_json::from_str(load_json_fixture("e621ng/post_id").as_str()).unwrap();
        let model: E621ngPost = json.into();
        assert_eq!(model.id().to_string(), model.id.to_string());
        assert_eq!(
            model.hash().as_ref().unwrap().to_string().as_str(),
            model.file.md5.as_str()
        );
        let images = model.images();
        assert_eq!(
            images.original.as_ref().unwrap().url.to_string().as_str(),
            model.file.url.as_ref().unwrap()
        );

        let tags = model.tags();
        assert!(tags.len() > 0);
        println!("{:p}, {:p} {:p}", &tags,
                 tags[0].as_ref(),
                 model.tags.general[0].as_str());
    }
}
