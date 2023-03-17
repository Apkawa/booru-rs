![ci-badge][] [![crates.io version]][crates.io link] 
# booru-rs
An async Booru client for Rust

##  Overview
The client currently supports engine:

- [x] [Danbooru](https://github.com/danbooru/danbooru)
    * danbooru.donmai.us
    * hijiribe.donmai.us
    * sonohara.donmai.us
- [x] danbooru v1
    * behoimi.org
- [x] Gelbooru 
    * gelbooru.com
- [x] Gelbooru v0.2
    * safebooru.org
- [x] [Moebooru](https://github.com/moebooru/moebooru)
    * konachan.com
    * konachan.net
    * yande.re
- [ ] [philomena](https://github.com/philomena-dev/philomena)
  * derpibooru.org
- [ ] Zerochain (=lainchan = vichan)
  * zerochan.net
  * kpop.asiachan.com
- [ ] [Shimmie](https://github.com/shish/shimmie2)
- [ ] [szurubooru](https://github.com/rr-/szurubooru)

[List of boorus sites](https://github.com/red-tails/list-of-boorus/)

## Example
```rust
let posts = GelbooruClient::builder()
    .tag("kafuu_chino")
    .tag("2girls")
    .rating(GelbooruRating::General)
    .sort(GelbooruSort::Score)
    .limit(5)
    .random(true)
    .blacklist_tag(GelbooruRating::Explicit)
    .get()
    .await
    .expect("There was an error retrieving posts from the API");
```

[ci-badge]: https://img.shields.io/github/actions/workflow/status/ajiiisai/booru-rs/ci.yml?branch=main
[crates.io link]: https://crates.io/crates/booru-rs
[crates.io version]: https://img.shields.io/crates/v/booru-rs.svg?style=flat-square
