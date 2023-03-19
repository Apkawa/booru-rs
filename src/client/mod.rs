pub mod generic;

#[cfg(feature = "danbooru")]
pub mod danbooru;
#[cfg(feature = "danbooru_v1")]
pub mod danbooru_v1;
#[cfg(feature = "e621ng")]
pub mod e621ng;
#[cfg(feature = "gelbooru")]
pub mod gelbooru;
#[cfg(feature = "gelbooru_v02")]
pub mod gelbooru_v0_2;
#[cfg(feature = "moebooru")]
pub mod moebooru;
#[cfg(feature = "philomena")]
pub mod philomena;
#[cfg(feature = "zerochan")]
pub mod zerochan;
