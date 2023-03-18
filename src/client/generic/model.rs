
// TODO
trait BooruPostModel {
    fn image_original_url(&self) -> String;
    fn image_sample_url(&self) -> String;
    fn image_preview_url(&self) -> String;
    fn source_url(&self) -> Option<String>;
    fn tags(&self) -> Vec<String>;
    fn artist(&self) -> Option<String>;
}