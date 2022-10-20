// imports
    use reqwest::{self,header::*};
    use serde::{Deserialize, Serialize};
    use image::{ImageBuffer,RgbImage, Rgb};
    use image;
    use imageproc::drawing::{draw_text_mut};
    use image::imageops::colorops::invert;
    use rusttype::{Font, Scale};
    use std;
    use rand::{self,Rng, distributions::Alphanumeric};
    use std::time::{SystemTime};
    use base64;
    use urlencoding;
    use hmac_sha1_compact::HMAC;
    use hmacsha1::hmac_sha1;

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
#[derive(Debug, Deserialize, Serialize)]
struct CredentialsObj{
    API_KEY:String,
    API_SECRET:String,
    ACCESS_TOKEN:String,
    ACCESS_TOKEN_SECRET:String
}

#[derive(Debug , Deserialize, Serialize)]
struct ImgData{
    image_type: String,
    w: u16,
    h: u16
}
#[derive(Debug , Deserialize, Serialize)]
struct InitResponse {
        media_id:u128,
        media_id_string:String,
        size:usize,
        expires_after_secs: usize,
        image:ImgData
}

const WIDTH: u32 = 1080;
const HEIGHT:u32 = 1920;


fn main() {
            
        let res = get_quote().unwrap();
        // creating an image


        // let img :RgbImage = CreateImage(&res); 
        // println!("{} \n \t {}",res.quote, res.author);


        // // now we have the image, We can post to twitter as an initial step towards
        // // making a big social profile
        // let mut tweet:RgbImage = ImageBuffer::new(WIDTH,HEIGHT);
        // tweet.clone_from(&img);

        // invert(&mut tweet);
        // img.save("output.png").unwrap();
        // tweet.save("output_tweet.png").unwrap();




        //posting to twitter
        let credentialsText = std::fs::read_to_string("src/credentials_tweeter.json")
        .unwrap();
        let credentials:CredentialsObj = serde_json::from_str(&credentialsText).unwrap();

        tweet_with(credentials);



        //let rustlang = egg_mode::user::show("rustlang", con_token).await.unwrap();

        //println!("{} (@{})", rustlang.name, rustlang.screen_name);

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
    let FONTSIZE = if (quote.quote.len() as f32 /17.0) > 15. {75/2} else {75};
    let letters_in_line =if (quote.quote.len() as f32 /17.0) > 15. {17*2} else {17} ;

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


    let mut current_letters_in_line = 0;

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

    return img;
}


fn tweet_with(credentials:CredentialsObj){
    /*

    from docs: 

    curl --request POST \

  --url 'https://api.twitter.com/1.1/statuses/update.json?status=Hello%20world' \

  --header 'authorization: OAuth oauth_consumer_key="CONSUMER_API_KEY", oauth_nonce="OAUTH_NONCE", oauth_signature="OAUTH_SIGNATURE", oauth_signature_method="HMAC-SHA1", oauth_timestamp="OAUTH_TIMESTAMP", oauth_token="ACCESS_TOKEN", oauth_version="1.0"' \
    */
    let client =  reqwest::blocking::Client::new();
    let url = "https://api.twitter.com/1.1/statuses/update.json";//"https://upload.twitter.com/1.1/media/upload.json";
    let img = std::fs::read("output_tweet.png").unwrap();
    let img_size = img.len();
    let img_size = img_size.to_string();
    let img_size = img_size.as_str();
    let parameters = [("include_entities","true"),("status","Hello Ladies + Gentlemen, a signed OAuth request!")]; //[("command","INIT" ),("total_bytes",img_size),                                    ("media_type","image/png")];




    // first we need to sign our request
    // Oauth Variables:
    let oauth_consumer_key = &credentials.API_KEY;
    let oauth_signature_method = "HMAC-SHA1";
    let oauth_token =  &credentials.ACCESS_TOKEN;
    let oauth_version = "1.0";

    // We now have all static oauth variables
    // we will start making our signature

    let mut parameters_string= String::new();
    for i in 0..parameters.len(){
        parameters_string += format!("{}={}",parameters[i].0,parameters[i].1).as_str();
        if i != parameters.len()-1 {parameters_string += "&"}
    }

    let timeStampINIT = get_timeStamp();
    let Oauth_nonceINIT =gen_oauth_nonce();

    let signatureINIT = get_signature(&oauth_consumer_key, &timeStampINIT, &Oauth_nonceINIT,
                                &oauth_signature_method.to_string(), &oauth_token,
                                &oauth_version.to_string(), &parameters_string, 
                                &url.to_string(), "POST".to_string(), &credentials);  

    // creating header
    let mut DST = String::from("OAuth ");
    DST += format!("{}=\"{}\", ", urlencoding::encode("oauth_consumer_key"), urlencoding::encode(&oauth_consumer_key) ).as_str();
    DST += format!("{}=\"{}\", ", urlencoding::encode("oauth_nonce"), urlencoding::encode(&Oauth_nonceINIT) ).as_str();
    DST += format!("{}=\"{}\", ", urlencoding::encode("oauth_signature"), urlencoding::encode(&signatureINIT) ).as_str();
    DST += format!("{}=\"{}\", ", urlencoding::encode("oauth_signature_method"), urlencoding::encode(&oauth_signature_method) ).as_str();
    DST += format!("{}=\"{}\", ", urlencoding::encode("oauth_timestamp"), timeStampINIT ).as_str();
    DST += format!("{}=\"{}\", ", urlencoding::encode("oauth_token"), urlencoding::encode(&oauth_token) ).as_str();
    DST += format!("{}=\"{}\"", urlencoding::encode("oauth_version"), urlencoding::encode(&oauth_version) ).as_str();


    println!("Consumer key: {}\nNonce: {}\nSignature: {}\nSignature method: {}\nTimestamp: {}\nToken: {}\nVersion: {}\n\n",
            oauth_consumer_key, Oauth_nonceINIT, signatureINIT,oauth_signature_method, timeStampINIT,
        oauth_token,oauth_version);

    let responseTest = client.post(url)
                                            .header(AUTHORIZATION, &DST)
                                            .query(&parameters)
                                            .send()
                                            .unwrap()
                                            .text().unwrap();

    println!("{} \n\n\n {}" ,&responseTest, DST);



}
fn gen_oauth_nonce() -> String{
    let randomStr = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(32)
    .map(char::from)
    .collect::<String>();
    let mut encoding = base64::encode(randomStr);
    encoding.retain(|c| c.is_alphanumeric());
    return encoding;
}

fn get_timeStamp()-> u128{
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as u128
}

fn get_signature(oauth_consumer_key:&String,timeStamp:&u128,oauth_nonce:&String,
                    oauth_signature_method:&String, oauth_token:&String, oauth_version:&String,
                    parametersString:&String, url:&String, method:String, credentials:&CredentialsObj) ->String{

    // following https://developer.twitter.com/en/docs/authentication/oauth-1-0a/creating-a-signature#f1
        let mut signature_base = String::new();
            signature_base += format!("oauth_consumer_key={}&",oauth_consumer_key).as_str();
            signature_base += format!("oauth_nonce={}&",oauth_nonce).as_str();
            signature_base += format!("oauth_signature_method={}&",oauth_signature_method).as_str();
            signature_base += format!("oauth_timestamp={}&",timeStamp).as_str();
            signature_base += format!("oauth_token={}&",oauth_token).as_str();
            signature_base += format!("oauth_version={}&",oauth_version).as_str();
            signature_base += parametersString.as_str();

            let mut signature_base_vec = signature_base.split("&").collect::<Vec<&str>>();
    signature_base_vec.sort_by(|a,b| a.to_lowercase().cmp(&b.to_lowercase()));
    let mut signature_base = String::new();
    for prop in signature_base_vec.into_iter() {
        let pair = prop.split("=").collect::<Vec<&str>>();
        let key = urlencoding::encode(pair[0]).into_owned();
        let value = urlencoding::encode(pair[1]).into_owned();
        signature_base += format!("{}={}&",key,value).as_mut_str();
    }
    

    signature_base = format!("{}&{}&{}",method,urlencoding::encode(url).into_owned(),
                        urlencoding::encode(&signature_base[0..signature_base.len()-1]).into_owned());
    println!("{} \n\n",signature_base);

    let signing_key = format!("{}&{}",
                urlencoding::encode(credentials.API_SECRET.as_str()),
                urlencoding::encode(credentials.ACCESS_TOKEN_SECRET.as_str()));


    let sign_hash =hmac_sha1(signing_key.as_bytes(),signature_base.as_bytes(), );


    // just making a test yo make sure its working
        
    let signature = base64::encode(sign_hash);
    
    return signature;
}