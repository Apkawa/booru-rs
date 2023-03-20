#[cfg(feature = "danbooru")]
#[cfg(test)]
mod danbooru {
    use crate::helpers::load_json_fixture;
    use booru_rs::client::danbooru::{DanbooruClient, DanbooruPost, DanbooruRating, DanbooruSort};
    use booru_rs::client::generic::{BooruClient, BooruOptionBuilder, BooruPostModel};

    #[test]
    fn get_posts_with_tag() {
        let posts = DanbooruClient::new()
            .url("https://testbooru.donmai.us")
            .tag("kafuu_chino")
            .get()
            .unwrap();

        assert!(!posts.is_empty());
    }

    #[test]
    fn get_posts_with_rating() {
        let posts = DanbooruClient::new()
            .url("https://testbooru.donmai.us")
            .tag("kafuu_chino")
            .rating(DanbooruRating::General)
            .get();

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[test]
    fn get_posts_with_sort() {
        let posts = DanbooruClient::new()
            .url("https://testbooru.donmai.us")
            .tag("kafuu_chino")
            .order(DanbooruSort::Rating)
            .get();

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[test]
    fn get_posts_with_blacklist_tag() {
        let posts = DanbooruClient::new()
            .url("https://testbooru.donmai.us")
            .tag("kafuu_chino")
            .blacklist_tag(DanbooruRating::Explicit)
            .get();

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[test]
    fn get_posts_with_limit() {
        let posts = DanbooruClient::new()
            .url("https://testbooru.donmai.us")
            .tag("kafuu_chino")
            .limit(3)
            .get();

        assert!(posts.is_ok());
        assert!(posts.unwrap().len() == 3);
    }

    #[test]
    fn get_posts_multiple_tags() {
        let posts = DanbooruClient::new()
            .url("https://testbooru.donmai.us")
            .tag("kafuu_chino")
            .tag("bangs")
            .limit(3)
            .get();

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[test]
    fn get_random_posts() {
        let posts = DanbooruClient::new()
            .url("https://testbooru.donmai.us")
            .tag("kafuu_chino")
            .random(true)
            .get();

        assert!(posts.is_ok());
        assert!(!posts.unwrap().is_empty());
    }

    #[test]
    fn get_post_by_id() {
        let post = DanbooruClient::new()
            .url("https://testbooru.donmai.us")
            .get_by_id(9423);

        assert!(post.is_ok());
        assert_eq!(
            "15a1b49c26f5c684807a2f0b838f9e4c",
            post.unwrap().md5.unwrap()
        );
    }

    #[test]
    fn parse_rating_tags() {
        assert_eq!("explicit", DanbooruRating::Explicit.to_string());
        assert_eq!("questionable", DanbooruRating::Questionable.to_string());
        assert_eq!("sensitive", DanbooruRating::Sensitive.to_string());
        assert_eq!("general", DanbooruRating::General.to_string());
    }

    #[test]
    fn parse_sort_tags() {
        assert_eq!("id", DanbooruSort::Id.to_string());
        assert_eq!("score", DanbooruSort::Score.to_string());
        assert_eq!("rating", DanbooruSort::Rating.to_string());
        assert_eq!("user", DanbooruSort::User.to_string());
        assert_eq!("height", DanbooruSort::Height.to_string());
        assert_eq!("width", DanbooruSort::Width.to_string());
        assert_eq!("source", DanbooruSort::Source.to_string());
        assert_eq!("updated", DanbooruSort::Updated.to_string());
    }

    #[test]
    fn posts_deserialize_json() {
        let json: Vec<DanbooruPost> =
            serde_json::from_str(load_json_fixture("danbooru/posts").as_str()).unwrap();
        assert_eq!(json.len(), 10);
    }

    #[test]
    fn post_deserialize_json() {
        let json: DanbooruPost =
            serde_json::from_str(load_json_fixture("danbooru/post_id").as_str()).unwrap();
        let model: DanbooruPost = json;
        assert_eq!(model.id, 6148688);
    }
    #[test]
    fn post_booru_model_trait() {
        let model: DanbooruPost =
            serde_json::from_str(load_json_fixture("danbooru/post_id").as_str()).unwrap();
        assert_eq!(model.id().to_string(), "6148688".to_string());
        assert_eq!(
            model.hash().as_ref().unwrap().to_string().as_str(),
            model.md5.as_ref().unwrap()
        );
        let images = model.images();
        assert_eq!(
            images.original.as_ref().unwrap().url.to_string().as_str(),
            model.file_url.as_ref().unwrap()
        );
        let characters = model.character();
        assert_eq!(characters.len(), 1)
    }
}
