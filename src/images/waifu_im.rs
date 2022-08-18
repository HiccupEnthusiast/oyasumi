use log::{debug, trace};
use reqwest::Client;
use serde_json::Value;

use crate::image::{Image, Builder};

pub async fn request_image(nsfw: &str) -> Result<Image, Box<dyn std::error::Error>> {
    debug!(target: "get_events", "Retreiving image from waifu.im");
    
    let client = Client::new();
    let url = format!("https://api.waifu.im/random/?is_nsfw={}", nsfw);
    let res = client.get(url).send().await?.text().await?;
    trace!("Response from the API: {}", &res);
    let json: Value = serde_json::from_str(&res)?;

    let img = Builder::new()
        .source(crate::image::Source::WaifuIm)
        .post_url(json["images"][0]["source"].as_str().unwrap().to_string())
        .img_url(json["images"][0]["url"].as_str().unwrap().to_string())
        .preview_url(json["images"][0]["preview_url"].as_str().unwrap().to_string())
        .extension(json["images"][0]["extension"].as_str().unwrap().to_string())
        .is_nsfw(json["images"][0]["is_nsfw"].as_bool().unwrap())
        .size(
            json["images"][0]["width"].as_u64().unwrap(),
            json["images"][0]["height"].as_u64().unwrap())
        .build();
        
    debug!("{}", img);

    Ok(img)
}