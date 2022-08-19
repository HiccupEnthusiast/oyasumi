//! Oyasumi is an wrapper for various APIs that relate to Animanga content.
//! 
//! This library aims to standarize the interactions with different APIs that 
//! can be used in projects by providing structures and functions to handle the
//! task of fetching, filtering and mapping the information avaliable from 
//! the APIs. 
//! 
//! A full list of the APIs and supported features this project uses can be found
//! at the [`apis`] section in this project's README.
//! 
//! [`Image`] is the struct that all images will be mapped to.
//! 
//! Important properties from this struct are `post_url` which is a link 
//! to the place where the image was found originally, ideally it corresponds to 
//! the source. `preview_url` which is a link to a dowscaled version of the image,
//! perfect for thumbnails and previews where the full resolution isn't necessary.
//! Lastly there's is `img_url` which is the link to the image in the API's database.
//! 
//! A more throughout explanation can be found in the [`examples`] directory of
//! the repository
//! 
//! ## Installation
//! 
//! To install this library add this to `Cargo.toml` 
//! 
//! ```toml
//! [dependencies]
//! oyasumi = "0.1"
//! ```
//! 
//! This library uses async functions, it is necessary to an async runtime. A
//! popular choice is [`tokio`], a minimum installation could look like
//! 
//! ```toml
//! # [dependencies]
//! tokio = {version = "1", features = ["macros", "signal", "rt-multi-thread"]}
//! ```
//! 
//! Additionally this library logs some of its activity for ease of debugging 
//! using the [`log`] crate.
//! 
//! ## Getting started
//! 
//! To get started you can get a random sfw image with `Image::fetch_sfw()` which
//! will get a random safe for work image from a random API 
//! 
//! ```rust
//! # use oyasumi::image::{Image, Builder};
//! # #[tokio::main]
//! # async fn main () {
//!     let img = Image::fetch_sfw().await;
//!     // Example data
//! #   let mut img = Builder::new().img_url("https://cdn.waifu.im/e6975bf3222e9074.jpg".to_string()).build();
//!     assert_eq!(img.img_url, "https://cdn.waifu.im/e6975bf3222e9074.jpg")
//! # }
//! ```
//! 
//! 
//! 
//! [`examples`]: https://github.com/HiccupEnthusiast/oyasumi/examples
//! [`apis`]: https://github.com/HiccupEnthusiast/oyasumi/#apis
//! [`Image`]: crate::image::Image
//! [`tokio`]: https://docs.rs/tokio/1/tokio/

pub mod image;
pub(crate) mod images;
