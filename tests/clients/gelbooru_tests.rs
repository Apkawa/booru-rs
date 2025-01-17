#[cfg(feature = "gelbooru")]
#[cfg(test)]
mod gelbooru {
    use booru_rs::client::gelbooru::model::{GelbooruPost, GelbooruResponse};
    use booru_rs::client::gelbooru::{
        model::{GelbooruRating, GelbooruSort},
        GelbooruClient,
    };
    use booru_rs::client::generic::{BooruClient, BooruOptionBuilder, BooruPostModel};

    use crate::helpers::{load_json_fixture, proxy};

    #[test]
    fn get_posts_with_tag() {
        let posts = GelbooruClient::new()
            .proxy(proxy())
            .tag("kafuu_chino")
            .get()
            .unwrap();

        assert!(!posts.is_empty());
    }

    #[test]
    fn get_posts_with_rating() {
        let posts = GelbooruClient::new()
            .proxy(proxy())
            .tag("kafuu_chino")
            .rating(GelbooruRating::General)
            .get()
            .unwrap();

        assert!(!posts.is_empty());
    }

    #[test]
    fn get_posts_with_sort() {
        let posts = GelbooruClient::new()
            .proxy(proxy())
            .tag("kafuu_chino")
            .order(GelbooruSort::Score)
            .get()
            .unwrap();

        assert!(!posts.is_empty());
    }

    #[test]
    fn get_posts_with_blacklist_tag() {
        let posts = GelbooruClient::new()
            .proxy(proxy())
            .tag("kafuu_chino")
            .blacklist_tag(GelbooruRating::Explicit)
            .get()
            .unwrap();

        assert!(!posts.is_empty());
    }

    #[test]
    fn get_posts_with_limit() {
        let posts = GelbooruClient::new()
            .proxy(proxy())
            .tag("kafuu_chino")
            .rating(GelbooruRating::General)
            .limit(3)
            .get();

        assert!(posts.is_ok());
        assert!(posts.unwrap().len() == 3);
    }

    #[test]
    fn get_posts_multiple_tags() {
        let posts = GelbooruClient::new()
            .proxy(proxy())
            .tag("1girl")
            .tag("cow_girl")
            .limit(3)
            .get();

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[test]
    fn get_random_posts() {
        let posts = GelbooruClient::new()
            .proxy(proxy())
            .tag("kafuu_chino")
            .random(true)
            .get();

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[test]
    fn get_post_by_id() {
        let post = GelbooruClient::new().proxy(proxy()).get_by_id(7898595);

        assert_eq!("e40b797a0e26755b2c0dd7a34d8c95ce", post.unwrap().md5);
    }

    #[test]
    fn parse_rating_tags() {
        assert_eq!("explicit", GelbooruRating::Explicit.to_string());
        assert_eq!("questionable", GelbooruRating::Questionable.to_string());
        assert_eq!("safe", GelbooruRating::Safe.to_string());
        assert_eq!("sensitive", GelbooruRating::Sensitive.to_string());
        assert_eq!("general", GelbooruRating::General.to_string());
    }

    #[test]
    fn parse_sort_tags() {
        assert_eq!("id", GelbooruSort::Id.to_string());
        assert_eq!("score", GelbooruSort::Score.to_string());
        assert_eq!("rating", GelbooruSort::Rating.to_string());
        assert_eq!("user", GelbooruSort::User.to_string());
        assert_eq!("height", GelbooruSort::Height.to_string());
        assert_eq!("width", GelbooruSort::Width.to_string());
        assert_eq!("source", GelbooruSort::Source.to_string());
        assert_eq!("updated", GelbooruSort::Updated.to_string());
    }

    #[test]
    fn posts_deserialize_json() {
        let json: GelbooruResponse =
            serde_json::from_str(load_json_fixture("gelbooru/posts").as_str()).unwrap();
        assert_eq!(json.posts.len(), 10);
        let models: Vec<GelbooruPost> = json.into();
        assert_eq!(models.len(), 10);
    }

    #[test]
    fn post_deserialize_json() {
        let json: GelbooruResponse =
            serde_json::from_str(load_json_fixture("gelbooru/post_id").as_str()).unwrap();
        let model: GelbooruPost = json.into();
        assert_eq!(model.id, 8297763);
        assert_eq!(model.created_at, "2023-03-04T09:16:56-06:00".to_string())
    }

    #[test]
    fn post_booru_model_trait() {
        let json: GelbooruResponse =
            serde_json::from_str(load_json_fixture("gelbooru/post_id").as_str()).unwrap();
        let model: GelbooruPost = json.into();
        assert_eq!(model.id().to_string(), model.id.to_string());
        assert_eq!(
            model.hash().as_ref().unwrap().to_string().as_str(),
            model.md5.as_str()
        );
        let images = model.images();
        assert_eq!(
            images.original.as_ref().unwrap().url.to_string(),
            model.file_url
        )
    }
}
