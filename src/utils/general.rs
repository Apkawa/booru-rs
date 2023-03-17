use reqwest::header::{self, HeaderMap};

// This is only here because of Danbooru, thanks Danbooru, really cool :)
pub fn get_default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static("PostmanRuntime/7.30.0"),
    );
    headers
}
