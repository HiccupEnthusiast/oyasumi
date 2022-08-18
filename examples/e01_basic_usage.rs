use oyasumi::image::{Image, Source, Builder};

// This library is async, use the runtime of your preference
// We'll use Tokio for the sake of our examples
#[tokio::main]
async fn main () -> Result<(), Box<dyn std::error::Error>> {

    // To get an image you can create it by providing a Source, 
    // here we will be requesting one from waifu.im API
    let image_from = Image::new(Source::WaifuIm).await;
    println!("Image from source: {}\n", image_from);

    // You can get the value of any value at any time by just accessing them
    println!("URL: {}\n", image_from.img_url);

    // If you want to get the basic structure of the Image struct 
    // you can create one with None as the source
    let empty = Image::new(Source::None).await;
    println!("Base image struct: {}\n", empty);

    // If you don't care about the source and just want a random image
    // You can use the random function to get one
    let random_image = Image::random().await;
    println!("Random image: {}\n", random_image);

    // You can also build your own Image, note that you don't need to
    // fill all the parameters if you use the build() funcion
    let built_image = Builder::new()
        .source(Source::None)
        .artist("Some awesome person".to_string())
        .character("My very own OC".to_string())
        .series("My works".to_string())
        .post_url("https://myownblog.com/my_works/42/".to_string())
        .img_url("https://myownblog.com/42/img/answer_to_life.png".to_string())
        .preview_url("https://myownblog.com/42/img/thumbnail.jpg".to_string())
        .extension("png".to_string())
        .size(1920, 1080)
        .build();
    println!("Built image: {}\n", built_image);

    // Image::random() and Image::new() don't discriminate between SFW and NSFW
    // works. You can filter them manually by accessing .is_nfsw property or 
    // you can use one of the following helper functions
    let _sfw_image = Image::fetch_sfw().await;
    let _nsfw_image = Image::fetch_nsfw().await;

    Ok(())
}