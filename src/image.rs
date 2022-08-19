use std::{fmt};
use serde::{Serialize, Deserialize};
use rand::{distributions::{Distribution, Standard}, Rng,};
use crate::images::*;

/// Data obtained from a image endpoint
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Image {
    /// What API was used to get the image
    pub source: Source,
    /// Name of the artist who made the image
    pub artist: String,
    /// Name of the main character in the image
    pub character: String,
    /// Name of the source material
    pub series: String,
    /// URL to the page where the image was taken, ideally it corresponds to
    /// the source. If it does or not depends in what the API considers as the
    /// "source" link
    pub post_url: String,
    /// URL to the fullres image in the API's database
    pub img_url: String,
    /// URL to a lighter and smaller version of the image, it has less resolution
    /// so it uses less data and loads faster.
    pub preview_url: String,
    /// What file format does the image have
    pub extension: Extension,
    /// If the image is Not Suitable For Work
    pub is_nsfw: bool,
    /// The size of the image in pixels (Width x Height)
    pub size: Size,
}

/// Builder helper to create a new Image struct, unnespecified fields will fallback
/// to a default value
#[derive(Debug)]
#[non_exhaustive]
pub struct Builder {
    /// What API was used to get the image
    pub source: Source,
    /// Name of the artist who made the image
    pub artist: String,
    /// Name of the main character in the image
    pub character: String,
    /// Name of the source material
    pub series: String,
    /// URL to the page where the image was taken, ideally it corresponds to
    /// the source. If it does or not depends in what the API considers as the
    /// "source" link
    pub post_url: String,
    /// URL to the fullres image in the API's database
    pub img_url: String,
    /// URL to a lighter and smaller version of the image, it has less resolution
    /// so it uses less data and loads faster.
    pub preview_url: String,
    /// What file format does the image have
    pub extension: Extension,
    /// If the image is Not Suitable For Work
    pub is_nsfw: bool,
    /// The size of the image in pixels (Width x Height)
    pub size: Size,
}

/// Enum contaning the possible APIs that this library can use. 
/// 
/// None can be used if you created the Image struct programatically
/// ```rust
/// # use oyasumi::image::Builder;
/// # use oyasumi::image::Source;
/// # use oyasumi::image::Image;
/// # #[tokio::main]
/// # async fn main () { 
///    let _my_image = Builder::new()
///         .source(Source::None)
///         // ...info about my drawing... //
///         .build(); 
/// #   assert!(true);
/// # }
/// ```
/// Or if you want to obtain the default values of the Image struct (equivalent
/// to `Image::default()`)
/// ```rust
/// # use oyasumi::image::{Image, Source};
/// # #[tokio::main]
/// # async fn main (){
///     let _default_img = Image::new(Source::None);
/// #   assert!(true);
/// # }
/// ```
#[derive(Clone, Copy, Default, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub enum Source {
    /// Default value if nothing is provided
    #[default]
    None,
    /// Images from https://waifu.im/
    WaifuIm,
}
/// Enum contaning possible file formats
#[derive(Clone, Copy, Default, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub enum Extension {
    /// Default if nothing provided or the file extension is unknown
    #[default]
    Unknown,
    Png,
    Jpg,
    Gif,
    Svg,
}
/// Struct containing the dimensions of the images 
#[derive(Clone, Copy, Default, Debug, Deserialize, Serialize)]
pub struct Size {
    pub width: u64,
    pub height: u64,
}

impl Image  {
    /// Get a random image from a random source, doesn't discriminate between SFW
    /// ### Usage
    /// ```rust
    /// # use oyasumi::image::Image;
    /// # #[tokio::main]
    /// # async fn main() {
    ///     let img = Image::random().await;
    ///     if img.is_nsfw {
    ///         // ..reject.. //
    ///     } else {
    ///         // ..accept.. //
    ///     }
    /// # assert!(true)
    /// # } 
    /// ```
    /// 
    pub async fn random() -> Image {
        let src: Source = rand::random();
        Image::new(src).await
    }
    /// Create an image from an specific source, doesn't discriminate between SFW and SFW
    /// Using `Source::None` will return default Image
    pub async fn new (src: Source) -> Image {
        Self::fetch_image(src, "random").await
    }
    /// Fetches a random image suitable for work from a random source
    pub async fn fetch_sfw() -> Image {
        let src: Source = rand::random();
        Self::fetch_image(src, "sfw").await
    }
    /// Fetches a random not suitable for work image from a random source
    pub async fn fetch_nsfw() -> Image {
        let src: Source = rand::random();
        Self::fetch_image(src, "nsfw").await
    }
    async fn fetch_image(src: Source, nsfw: &str) -> Image {
        match src {
            Source::None => Image::default(),
            Source::WaifuIm => {
                let nsfw_tag = match nsfw {
                    "random" => "null",
                    "nsfw" => "true",
                    _ => "false"
                };
                match waifu_im::request_image(nsfw_tag).await {
                    Ok(img) => img,
                    _ => todo!(), //retry function
                }
            }
        }
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