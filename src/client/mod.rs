pub mod generic;

#[cfg(feature = "danbooru")]
pub mod danbooru;
#[cfg(feature = "danbooru")]
pub mod danbooru_v1;
#[cfg(feature = "gelbooru")]
pub mod gelbooru;
#[cfg(feature = "gelbooru")]
pub mod gelbooru_v0_2;
#[cfg(feature = "moebooru")]
pub mod moebooru;

pub mod philomena;
