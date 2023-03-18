![ci-badge][] [![crates.io version]][crates.io link]

# booru-rs

An async Booru client for Rust

## Overview

The client currently supports engine:

- [x] [Danbooru](https://github.com/danbooru/danbooru)
    * danbooru.donmai.us
    * hijiribe.donmai.us
    * sonohara.donmai.us
- [x] danbooru v1
    * behoimi.org
- [x] Gelbooru
    * gelbooru.com
- [x] Gelbooru v0.2 (Gelbooru v0.1 no have api)
    * safebooru.org
    * realbooru.com
    * rule34.xxx
    * xbooru.com
    * tbib.org
    * hypnohub.net
    * mspabooru.com
- [x] [Moebooru](https://github.com/moebooru/moebooru)
    * konachan.com
    * konachan.net
    * yande.re
    * sakugabooru.com
- [x] [MyImouto](https://github.com/Yushe/myimouto-plus) Compatibly with Moebooru
    * lolibooru.moe
- [x] [philomena](https://github.com/philomena-dev/philomena)
    * derpibooru.org
    * furbooru.org
    * memebooru.com
    * tantabus.art
- [x] Zerochan (=lainchan = vichan)
    * zerochan.net
    * kpop.asiachan.com
- [x] [e621ng](https://e621.net/help/api)
    * e621.net
    * e926.net
- [ ] ~~[Shimmie](https://github.com/shish/shimmie2)~~ !NO HAVE API!
    * shimmie.shishnet.org
    * booru.soy
    * (pokeplayer.com)[https://pokeplayer.com/shimmie/]
    * booru.newblood.games
- [ ] [szurubooru](https://github.com/rr-/szurubooru)

[List of boorus sites](https://github.com/red-tails/list-of-boorus/)

## Features

* [x] Posts
* [x] Post by id
* [ ] Tag list/search
* [ ] Async

## Example

```rust
use booru_rs::client::danbooru::{
    DanbooruClient,
    DanbooruRating, DanbooruSort, DanbooruPost
};
use booru_rs::client::generic::{BooruClient, BooruClientBuilder};

#[test]
fn get_posts_with_tag() {
    let posts = DanbooruClient::builder()
        .default_url("https://testbooru.donmai.us")
        .tag("kafuu_chino")
        .tag("2girls")
        .rating(DanbooruRating::General)
        .sort(DanbooruSort::Score)
        .limit(5)
        .build()
        .get()
        .unwrap();

    assert!(!posts.is_empty());
}

#[test]
fn get_post_by_id() {
    let post = DanbooruClient::builder()
        .default_url("https://testbooru.donmai.us")
        .build()
        .get_by_id(9423)
        .unwrap();

    assert_eq!(
        post.id, 9423
    );
}
```

[ci-badge]: https://img.shields.io/github/actions/workflow/status/ajiiisai/booru-rs/ci.yml?branch=main

[crates.io link]: https://crates.io/crates/booru-rs

[crates.io version]: https://img.shields.io/crates/v/booru-rs.svg?style=flat-square

## Similar projects

* https://github.com/sinkaroid/booru
* https://github.com/Shiroechi/BooruDex
