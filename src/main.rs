use reqwest::{self};
use serde::{Deserialize, Serialize};
use image::{ImageBuffer,RgbImage};
#[derive(Debug, Deserialize, Serialize)]
struct Quote{
    anime:String,
    quote:String,
    character:String,
}

const WIDTH: u32 = 1080;
const HEIGHT:u32 = 1920;

fn main() {
            
        let res = get_quote().unwrap();

        // creating an image
        let mut img:RgbImage = ImageBuffer::new(WIDTH,HEIGHT);

        img.save("./output.png").unwrap();

        println!("{}",res.quote);

}


/**
 * get_quote makes a GET request to an API and retrieve a random quote
 */
fn get_quote() -> Result<Quote, reqwest::Error> {
    let url = "https://animechan.vercel.app/api/random";
    let res:Quote =reqwest::blocking::get(url)?
                        .json()?;

    //let quote_obj :Quote = serde_json::from_str(res.as_str())?;
    

    Ok(res)
}
