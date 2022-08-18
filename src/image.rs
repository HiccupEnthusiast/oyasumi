use std::{fmt};
use serde::Serialize;
use rand::{distributions::{Distribution, Standard}, Rng,};
use crate::images::*;

#[derive(Clone, Default, Debug, Serialize)]
pub struct Image {
    pub source: Source,
    pub artist: String,
    pub character: String,
    pub series: String,
    pub post_url: String,
    pub img_url: String,
    pub preview_url: String,
    pub extension: Extension,
    pub is_nsfw: bool,
    pub size: Size,
}
#[derive(Debug)]
pub struct Builder {
    pub source: Source,
    pub artist: String,
    pub character: String,
    pub series: String,
    pub post_url: String,
    pub img_url: String,
    pub preview_url: String,
    pub extension: Extension,
    pub is_nsfw: bool,
    pub size: Size,
}


#[derive(Clone, Copy, Default, Debug, Serialize)]
pub enum Source {
    #[default]
    None,
    WaifuIm,
}
#[derive(Clone, Copy, Default, Debug, Serialize)]
pub enum Extension {
    #[default]
    Unknown,
    Png,
    Jpg,
    Gif,
    Svg,
}
#[derive(Clone, Copy, Default, Debug, Serialize)]
pub struct Size {
    pub width: u64,
    pub height: u64,
}

impl Image  {
    pub async fn new (src: Source) -> Image {
        match src {
            Source::None => Image::default(),
            Source::WaifuIm => {
                match waifu_im::request_image().await {
                    Ok(img) => img,
                    _ => todo!(),
                }
            },
        }
    }
    pub async fn random() -> Image {
        let src: Source = rand::random();
        Image::new(src).await
    }
}

impl Builder  {
    #[must_use]
    pub fn new() -> Self {
        Builder::default()
    }
    #[must_use]
    pub fn source(mut self, src: Source) -> Self {
        self.source = src;
        self
    }
    #[must_use]
    pub fn artist(mut self, artist: String) -> Self {
        self.artist = artist;
        self
    }
    #[must_use]
    pub fn character(mut self, character: String) -> Self {
        self.character = character;
        self
    }
    #[must_use]
    pub fn series(mut self, series: String) -> Self {
        self.series = series;
        self
    }
    #[must_use]
    pub fn post_url(mut self, url: String) -> Self {
        self.post_url = url;
        self
    }
    #[must_use]
    pub fn img_url(mut self, url: String) -> Self {
        self.img_url = url;
        self
    }
    #[must_use]
    pub fn preview_url(mut self, url: String) -> Self {
        self.preview_url = url;
        self
    }
    #[must_use]
    pub fn extension(mut self, ext : String) -> Self {       
        match ext.trim().to_lowercase().replace(".", "").as_str() {
            "jpg" => {self.extension = Extension::Jpg; self},
            "jpeg" => {self.extension = Extension::Jpg; self},
            "png" => {self.extension = Extension::Png; self},
            "gif" => {self.extension = Extension::Gif; self},
            _ => {self}
        }
    }
    #[must_use]
    pub fn is_nsfw(mut self, is_nsfw: bool) -> Self {
        self.is_nsfw = is_nsfw;
        self
    }
    #[must_use]
    pub fn size(mut self, width: u64, height: u64) -> Self {
        self.size = Size {width, height};
        self
    }
    #[must_use]
    pub fn build (self) -> Image {
        Image {
            source: self.source,
            artist: self.artist,
            character: self.character,
            series: self.series,
            post_url: self.post_url,
            img_url: self.img_url,
            preview_url: self.preview_url,
            extension: self.extension,
            is_nsfw: self.is_nsfw,
            size: self.size,
        }
    }
}


impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, 
            "Image fetched from: {:?}, \
            Artist: {}, \
            Character: {}, \
            Series: {}, \
            Post URL: {}, \
            Image URL: {}, \
            Preview URL: {}, \
            File type: {:?}, \
            Is NSFW: {}, \
            Size: {}", 
            self.source, self.artist, self.character, 
            self.series, self.post_url, self.img_url,
            self.preview_url, self.extension, self.is_nsfw,
            self.size)
    }
}
impl Distribution<Source> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Source {
        match rng.gen_range(0..=2) {
            _ => Source::WaifuIm
        }
    }
}
impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}
impl Default for Builder {
    fn default() -> Self {
        Self {
            source: Source::None,
            artist: "Unknown".to_string(),
            character: "Unknown".to_string(),
            series: "Unknown".to_string(),
            post_url: "N/A".to_string(),
            img_url: "N/A".to_string(),
            preview_url: "N/A".to_string(),
            extension: Extension::Unknown,
            is_nsfw: false,
            size: Size { width: (0), height: (0) },
        }
    } 
}