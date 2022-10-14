use reqwest::{self};
// use serde::__private::de::IdentifierDeserializer;
use serde::{Deserialize, Serialize};
use image::{ImageBuffer,RgbImage, Rgb};
use image;
use imageproc::drawing::{draw_text_mut};
use rusttype::{Font, Scale};
#[derive(Debug, Deserialize, Serialize)]
struct Quote{
    //anime:String,
    quote:String,
    author:String,
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonQuote{
    result:Quote
}

const WIDTH: u32 = 1080;
const HEIGHT:u32 = 1920;
const FONTSIZE:u32=75;

fn main() {
            
        let res = get_quote().unwrap();
        // creating an image


        let img :RgbImage = CreateImage(&res); 
        println!("{} \n \t {}",res.quote, res.author);

        img.save("output.png").unwrap();

}


/**
 * get_quote makes a GET request to an API and retrieve a random quote
 */
fn get_quote() -> Result<Quote, reqwest::Error> {
    let url = "https://www.abbreviations.com/services/v2/quotes.php?uid=10998&tokenid=uNMIuLwQmsm1lf7W&format=json";
    let res:JsonQuote =reqwest::blocking::get(url)?
                        .json()?;
    // let res:JsonQuote = JsonQuote { result: Quote { 
    //                             quote: String::from("Then life makes us forget, that it ends. then see what it be, then do what it wants"),
    //                             author:String::from("Me")} };
    //let quote_obj :Quote = serde_json::from_str(res.as_str())?;
    

    Ok(res.result)
}
/**
 * createImage generates the image with the given quote
 */

fn CreateImage(quote:&Quote) -> RgbImage{
    let mut img:RgbImage = ImageBuffer::new(WIDTH,HEIGHT);
    let font = Vec::from(include_bytes!("Raleway-Medium.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();


    let mut scale = Scale{
            x:(FONTSIZE) as f32,
            y:(FONTSIZE as f32 *1.1)
    };

    

    // we need to add space before we go out of bounds
    // we will do that be calculating ( (sentence_length*FONTSIZE)%(width- 2*padding) ) / FONTSIZE
    // this will give us the number of letters in each line, but I am not sure why it doesnt work
    let sentence_length = quote.quote.len() as u32;
    let padding = WIDTH/8;
    let h_padding = HEIGHT/3;

    let quote_= &quote.quote.replace("\n", " ");

    let letters_in_line = 20;
    println!("{}",sentence_length);
    let mut current_letters_in_line = 0;
    let lines_in_text= sentence_length%letters_in_line;
    let words = quote_.split(" ");
    let mut line_num = 0;

    if letters_in_line !=0{
        let mut text = String::new();
            for word in words{
                
                current_letters_in_line += word.len() as u32;
                
                if current_letters_in_line > letters_in_line{
                    
                    draw_text_mut(&mut img, Rgb([255u8, 255u8, 255u8]),
                    padding as i32, ((h_padding) + line_num*FONTSIZE) as i32, scale, &font, &text);
                    
                    line_num +=1;
                    text = String::new();
                    current_letters_in_line %= letters_in_line
                }
                text += " ";
                text += word;
                
            }
            draw_text_mut(&mut img, Rgb([255u8, 255u8, 255u8]),
                    padding as i32, ((h_padding) + line_num*FONTSIZE) as i32, scale, &font, &text)


    }else{
        let text= quote_.clone();
        draw_text_mut(&mut img, Rgb([255u8, 255u8, 255u8]),
                     padding as i32, ((h_padding) + line_num*FONTSIZE) as i32,
                     scale, &font, &text);
    }
    let mut author="--".to_string();
    author += quote.author.as_str();
    draw_text_mut(&mut img, Rgb([255u8, 255u8, 255u8]),
                     padding as i32,((h_padding) + (line_num+3)*FONTSIZE) as i32,
                     scale, &font,&author);


    scale.x *= 2.0;
    scale.y *= 2.0;
    draw_text_mut(&mut img, Rgb([255u8, 255u8, 255u8]),
                     padding as i32, (HEIGHT/5) as i32,
                     scale, &font, "for you ...");

    
    return  img;
}