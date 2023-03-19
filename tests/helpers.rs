use std::env;
use std::fs::read_to_string;
use test_helpers::fixture::get_git_root;

pub fn proxy() -> Option<String> {
    env::var("HTTP_PROXY").ok()
}

pub fn load_fixture(name: &str) -> String {
    let fixture_path = get_git_root()
        .unwrap()
        .join(format!("tests/fixtures/{name}"));
    dbg!(&fixture_path);
    read_to_string(fixture_path).unwrap()
}

pub fn load_json_fixture(name: &str) -> String {
    load_fixture(format!("{name}.json").as_str())
}
