use oyasumi::image::{Image, Source};

// This library is async, use the runtime of your preference
// We'll use Tokio for the sake of our examples
#[tokio::main]
async fn main () -> Result<(), Box<dyn std::error::Error>> {

    // To get an image you can create it by providing a Source, 
    // here we will be requesting one from waifu.im API
    let image_from = Image::new(Source::WaifuIm).await;
    println!("Image from source: {}", image_from);

    // If you want to get the basic structure of the Image struct 
    // you can create one with None as the source
    let empty = Image::new(Source::None).await;
    println!("Base image object: {}", empty);

    // If you don't care about the source and just want a random image
    // You can use the random function to get one
    let random_image = Image::random().await;
    println!("Random image: {}", random_image);

    Ok(())
}