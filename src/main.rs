#![allow(non_snake_case)]
// imports
    use reqwest::{self,header::*};
    use serde::{Deserialize, Serialize};
    use image::{ImageBuffer,RgbImage, Rgb};
    use image;
    use imageproc::drawing::{draw_text_mut};
    use image::imageops::colorops::invert;
    use rusttype::{Font, Scale};
    use std::{self, io, fs, thread, time};
    use std::io::Write;
    use rand::{self,Rng, distributions::Alphanumeric};
    use std::time::{SystemTime};
    use base64;
    use urlencoding;
    use hmacsha1::hmac_sha1;
    use webbrowser;
    use rdev::{simulate, Button, EventType, Key, SimulateError};
    use cli_clipboard::{ClipboardContext, ClipboardProvider};



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

// twitter stuff
#[derive(Debug, Deserialize, Serialize)]
struct CredentialsObj{
    API_KEY:String,
    API_SECRET:String,
    ACCESS_TOKEN:String,
    ACCESS_TOKEN_SECRET:String
}

#[derive(Debug , Deserialize, Serialize)]
struct InitResponse {
        media_id:u128,
        media_id_string:String,
        expires_after_secs: usize,
}


const WIDTH: u32 = 1080;
const HEIGHT:u32 = 1920;

const NUMBER_OF_TWEETS :i32 = 50;


fn main() {

        let res = Quote { quote: String::new(), author: String::new() };//get_quote().unwrap();
        
        // creating an image

        let mut count =0;
        loop {
        println!("==================\nTweet #{} : \n",count+1);

        let img :RgbImage = CreateImage(&res, true); 

        println!("Image created");

        // now we have the image, We can post to twitter as an initial step towards
        // making a big social profile

        let img = fs::read("output_tweet.jpeg").unwrap();
        let img_Name = format!("quote_{:03}",count+1); // it gets a unique string so its good

        println!("saving to quotes/{}.jpeg",img_Name);
        let _ = fs::copy("output_tweet.jpeg", format!("quotes/{}.jpeg",img_Name).as_str());

        //posting to twitter
        let credentialsText = std::fs::read_to_string("src/credentials.json")
        .unwrap();
        let credentials:CredentialsObj = serde_json::from_str(&credentialsText).unwrap();


        
        // now we will use ffmpeg to make a video out of the images
        // make sure to download ffmpeg and add it to path or add it in the same file

        
        println!("Stating twitter upload...");

        tweet_with(credentials,"output_tweet.jpeg");

        

        



        // delete video
        //fs::remove_file(format!("quotes/{}.mp4",img_Name)).unwrap();


        //println!("{}.mp4 deleted",img_Name);

        if count+1 == NUMBER_OF_TWEETS{
            break;
        }else{
            count+=1
        }
       
        }

        let command = format!("ffmpeg -framerate 1/5 -i quotes/quote_%3d.jpeg -c:v libx264 -r 30 -pix_fmt yuv420p quotes/quotes.mp4");
        println!("Creating video with fmmpeg...");

        let _ = std::process::Command::new("cmd")
        .args(["/C",command.as_str()])
        .output()
        .expect("Something went wrong");

        println!("uploading to youtube...");
        println!("Removing jpeg...");
        for i in 0..NUMBER_OF_TWEETS{
            let _ = fs::remove_file(format!("quotes/quote_{:03}.jpeg",i+1));
        }        

        upload_to_youtube(format!("quotes\\quotes.mp4").as_str());

        // let rustlang = egg_mode::user::show("rustlang", con_token).await.unwrap();

        //println!("{} (@{})", rustlang.name, rustlang.screen_name);

}



/**
 * get_quote makes a GET request to an API and retrieve a random quote
 */
fn _get_quote() -> Result<Quote, reqwest::Error> {
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


 // Tweeter Oauth 1.0 stuff
fn CreateImage(quote:&Quote, inspiro:bool) -> RgbImage{
    if inspiro{
        let imgurl = reqwest::blocking::get("https://inspirobot.me/api?generate=true")
                                                        .unwrap().text().unwrap();
        let img = reqwest::blocking::get(imgurl)
                                                .unwrap().bytes().unwrap();
        
        std::fs::write("output_tweet.jpeg", &img).unwrap_or(());
        
        // let file = File::create("test_tweet.png").unwrap();
        // let ref mut buff = BufWriter::new(file);
        // let encoder = image::codecs::png::PngEncoder::new(buff);
        // encoder.encode(&img, 650, 650, image::ColorType::Rgb16);
        // let text = ocr("test_tweet.png").unwrap_or(String::from("Read this quote"));

        //println!("{}" , text);
        
    }


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
    // let sentence_length = quote.quote.len() as u32;
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


fn tweet_with(credentials:CredentialsObj, filename:&str){
    /*

    from docs: 
        https://developer.twitter.com/en/docs/twitter-api/v1/media/upload-media/api-reference/post-media-upload-init
        https://developer.twitter.com/en/docs/twitter-api/v1/media/upload-media/api-reference/post-media-upload-append
        https://developer.twitter.com/en/docs/twitter-api/v1/media/upload-media/api-reference/post-media-upload-finalize

    curl --request POST \

  --url 'https://api.twitter.com/1.1/statuses/update.json?status=Hello%20world' \

  --header 'authorization: OAuth oauth_consumer_key="CONSUMER_API_KEY", oauth_nonce="OAUTH_NONCE", oauth_signature="OAUTH_SIGNATURE", oauth_signature_method="HMAC-SHA1", oauth_timestamp="OAUTH_TIMESTAMP", oauth_token="ACCESS_TOKEN", oauth_version="1.0"' \
    */
    println!("Initialising upload ");

    let client =  reqwest::blocking::Client::new();
    let url = "https://upload.twitter.com/1.1/media/upload.json";
    let img = std::fs::read(filename.to_owned()).unwrap();
    

    let img_size = img.len();
    let img_size = img_size.to_string();
    let img_size = img_size.as_str();
    let parameters = [("command","INIT" ),("total_bytes",img_size),("media_type","tweet_image") ,("media_category","tweet_image")];





    // first we need to sign our request
    // Oauth Variables:
    let oauth_consumer_key = &credentials.API_KEY;
    let oauth_signature_method = "HMAC-SHA1";
    let oauth_token =  &credentials.ACCESS_TOKEN;
    let oauth_version = "1.0";

    // We now have all static oauth variables
    // we will start making our signature
    
    let parameters_INIT= stringify_parameters(&parameters);

    let timeStampINIT = get_timeStamp();
    let Oauth_nonceINIT =gen_oauth_nonce();

    let signatureINIT = get_signature(&oauth_consumer_key, &timeStampINIT, &Oauth_nonceINIT,
                                &oauth_signature_method.to_string(), &oauth_token,
                                &oauth_version.to_string(), &parameters_INIT, 
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



        //initialising
    let InitResponse : InitResponse= client.post(url.to_owned() )
                                            .header(AUTHORIZATION, &DST)
                                            .query(&parameters)
                                            .send()
                                            .unwrap()
                                            .json().unwrap();

    let media_id = InitResponse.media_id;
    let media_id = media_id.to_string();
    let chunks = img.chunks(512);
    let chunks_num:usize = chunks.len();
    let mut idx = 0;
    let mut loading = 0;
    println!("Uploading...");
    print!("[");
    for chunk in chunks{
                
            let imgdata = base64::encode(chunk);
            let imgdata = imgdata.as_str();
            let segment_index = idx.to_string();

            // let imgdataSTR = &&img.into_iter().map(|c| char::from(c)).collect::<String>();            
            let parameters_append = [ ("command","APPEND"), ("media_id",media_id.as_str()),
                                            ("media_data",imgdata),("segment_index", segment_index.as_str())];
            
            
            let parameters_appendSTR = stringify_parameters(&parameters_append);
            
            let timeStamp_append = get_timeStamp();
            let Oauth_nonce_append =gen_oauth_nonce();

            let signature_append = get_signature(&oauth_consumer_key, &timeStamp_append, &Oauth_nonce_append,
                                        &oauth_signature_method.to_string(), &oauth_token,
                                        &oauth_version.to_string(), &parameters_appendSTR, 
                                        &url.to_string(), "POST".to_string(), &credentials);

            
            // creating OAuth header
            let mut DST = String::from("OAuth ");
            DST += format!("{}=\"{}\", ", urlencoding::encode("oauth_consumer_key"), urlencoding::encode(&oauth_consumer_key) ).as_str();
            DST += format!("{}=\"{}\", ", urlencoding::encode("oauth_nonce"), urlencoding::encode(&Oauth_nonce_append) ).as_str();
            DST += format!("{}=\"{}\", ", urlencoding::encode("oauth_signature"), urlencoding::encode(&signature_append) ).as_str();
            DST += format!("{}=\"{}\", ", urlencoding::encode("oauth_signature_method"), urlencoding::encode(&oauth_signature_method) ).as_str();
            DST += format!("{}=\"{}\", ", urlencoding::encode("oauth_timestamp"), timeStamp_append ).as_str();
            DST += format!("{}=\"{}\", ", urlencoding::encode("oauth_token"), urlencoding::encode(&oauth_token) ).as_str();
            DST += format!("{}=\"{}\"", urlencoding::encode("oauth_version"), urlencoding::encode(&oauth_version) ).as_str();


                // appending
            let _append_response= client.post(url.to_owned() )
                                                    .header(AUTHORIZATION, &DST)
                                                    .query(&parameters_append)
                                                    .send();
            idx += 1;
            // print loading bar
            // 20 / chunks_num = (idx+1) / x
            // x = (idx+1) * chunks_num / 20
            let loading_percent = ( (idx+1)* 100 )/ chunks_num;

            

            if loading_percent % 5 == 0 && loading != loading_percent{
                loading = loading_percent;
                print!("#");
                std::io::stdout().flush().unwrap();
            }
            
            
    }
    println!("]");


    //finalizing
    let parameters_final = [ ("command","FINALIZE"), ("media_id",media_id.as_str())];
    let parameters_finalSTR = stringify_parameters(&parameters_final);

    let timeStamp_final = get_timeStamp();
    let Oauth_nonce_final =gen_oauth_nonce();

    let signature_final = get_signature(&oauth_consumer_key, &timeStamp_final, &Oauth_nonce_final,
        &oauth_signature_method.to_string(), &oauth_token,
        &oauth_version.to_string(), &parameters_finalSTR, 
        &url.to_string(), "POST".to_string(), &credentials);


            // creating OAuth header
    let mut DST = String::from("OAuth ");
    DST += format!("{}=\"{}\", ", urlencoding::encode("oauth_consumer_key"), urlencoding::encode(&oauth_consumer_key) ).as_str();
    DST += format!("{}=\"{}\", ", urlencoding::encode("oauth_nonce"), urlencoding::encode(&Oauth_nonce_final) ).as_str();
    DST += format!("{}=\"{}\", ", urlencoding::encode("oauth_signature"), urlencoding::encode(&signature_final) ).as_str();
    DST += format!("{}=\"{}\", ", urlencoding::encode("oauth_signature_method"), urlencoding::encode(&oauth_signature_method) ).as_str();
    DST += format!("{}=\"{}\", ", urlencoding::encode("oauth_timestamp"), timeStamp_final ).as_str();
    DST += format!("{}=\"{}\", ", urlencoding::encode("oauth_token"), urlencoding::encode(&oauth_token) ).as_str();
    DST += format!("{}=\"{}\"", urlencoding::encode("oauth_version"), urlencoding::encode(&oauth_version) ).as_str();
    
    let _Final_Response = client.post(url.to_owned() )
                                    .header(AUTHORIZATION, &DST)
                                    .query(&parameters_final)
                                    .send()
                                    .unwrap()
                                    .text().unwrap();

    println!("Upload finished");

    
    // thats it we uploaded the image, lets tweet

    let url = "https://api.twitter.com/1.1/statuses/update.json";
    let parameters = [("status","AI quotes (inspirobot)" ), ("media_ids",media_id.as_str())];

    let parameters_tweet= stringify_parameters(&parameters);

    let timeStamp_tweet = get_timeStamp();
    let Oauth_nonce_tweet =gen_oauth_nonce();
    
    
    let signature_tweet = get_signature(&oauth_consumer_key, &timeStamp_tweet, &Oauth_nonce_tweet,
        &oauth_signature_method.to_string(), &oauth_token,
        &oauth_version.to_string(), &parameters_tweet, 
        &url.to_string(), "POST".to_string(), &credentials);  

// creating header
let mut DST = String::from("OAuth ");
DST += format!("{}=\"{}\", ", urlencoding::encode("oauth_consumer_key"), urlencoding::encode(&oauth_consumer_key) ).as_str();
DST += format!("{}=\"{}\", ", urlencoding::encode("oauth_nonce"), urlencoding::encode(&Oauth_nonce_tweet) ).as_str();
DST += format!("{}=\"{}\", ", urlencoding::encode("oauth_signature"), urlencoding::encode(&signature_tweet) ).as_str();
DST += format!("{}=\"{}\", ", urlencoding::encode("oauth_signature_method"), urlencoding::encode(&oauth_signature_method) ).as_str();
DST += format!("{}=\"{}\", ", urlencoding::encode("oauth_timestamp"), timeStamp_tweet ).as_str();
DST += format!("{}=\"{}\", ", urlencoding::encode("oauth_token"), urlencoding::encode(&oauth_token) ).as_str();
DST += format!("{}=\"{}\"", urlencoding::encode("oauth_version"), urlencoding::encode(&oauth_version) ).as_str();


//initialising
println!("Tweeting...");
let _tweet_Response = client.post(url.to_owned() )
                    .header(AUTHORIZATION, &DST)
                    .query(&parameters)
                    .send()
                    .unwrap()
                    .text().unwrap();
println!("Tweet finished");


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
            signature_base += format!("oauth_consumer_key={}&",urlencoding::encode(oauth_consumer_key) ).as_str();
            signature_base += format!("oauth_nonce={}&", urlencoding::encode(oauth_nonce)).as_str();
            signature_base += format!("oauth_signature_method={}&",urlencoding::encode(oauth_signature_method) ).as_str();
            signature_base += format!("oauth_timestamp={}&",timeStamp).as_str();
            signature_base += format!("oauth_token={}&",urlencoding::encode(oauth_token) ).as_str();
            signature_base += format!("oauth_version={}&",urlencoding::encode(oauth_version) ).as_str();
            signature_base += parametersString.as_str();

            let mut signature_base_vec = signature_base.split("&").collect::<Vec<&str>>();
    signature_base_vec.sort_by(|a,b| a.to_lowercase().cmp(&b.to_lowercase()));
    let mut signature_base = String::new();
    for prop in signature_base_vec.into_iter() {
        let pair = prop.split("=").collect::<Vec<&str>>();
        if pair.len()==1{panic!("Something is not right:")}
        let key = pair[0];
        let value = pair[1];
        signature_base += format!("{}={}&",key,value).as_mut_str();
    }
    

    signature_base = format!("{}&{}&{}",method,urlencoding::encode(url).into_owned(),
                        urlencoding::encode(&signature_base[0..signature_base.len()-1]).into_owned());
    //println!("{} \n\n",signature_base);

    let signing_key = format!("{}&{}",
                urlencoding::encode(credentials.API_SECRET.as_str()),
                urlencoding::encode(credentials.ACCESS_TOKEN_SECRET.as_str()));


    let sign_hash =hmac_sha1(signing_key.as_bytes(),signature_base.as_bytes(), );


    // just making a test yo make sure its working
        
    let signature = base64::encode(sign_hash);
    
    return signature;
}

fn stringify_parameters (parameters : &[(&str,&str)]) ->String{
    let mut parameters_string= String::new();
    for i in 0..parameters.len(){
        parameters_string += format!("{}={}",urlencoding::encode(parameters[i].0) ,urlencoding::encode(parameters[i].1)).as_str();
        if i != parameters.len()-1 {parameters_string += "&"}
    }
   
    return parameters_string;
}


// youtube upload with GUI cuz lazy to do another Oauth1.0a

fn upload_to_youtube(filename:&str){

    webbrowser::open("https://www.youtube.com/upload").unwrap();

    wait(5);


    // using clipboard to write easily
    let mut ctx = ClipboardContext::new().unwrap();

    // data we will need
    let file_path = std::env::current_dir().unwrap().display().to_string() + "\\" + filename;

    

        
    // click upload

    do_action(&EventType::MouseMove { x: 729., y: 600. });
    do_action(&EventType::ButtonPress(Button::Left));
    do_action(&EventType::ButtonRelease(Button::Left));

    // going to file name text area
    do_action(&EventType::MouseMove { x: 529., y: 720. });
    do_action(&EventType::ButtonPress(Button::Left));
    do_action(&EventType::ButtonRelease(Button::Left));
    
    wait(1);

    // pasting the file_path variable
    ctx.set_contents(file_path.to_owned()).unwrap();
    do_action(&EventType::ButtonPress(Button::Right));
    do_action(&EventType::ButtonRelease(Button::Right));

    do_action(&EventType::KeyPress(Key::DownArrow));
    do_action(&EventType::KeyPress(Key::DownArrow));
    do_action(&EventType::KeyPress(Key::DownArrow));
    do_action(&EventType::KeyPress(Key::DownArrow));
    do_action(&EventType::KeyPress(Key::Return));

    // click select
    do_action(&EventType::KeyPress(Key::Return));

    wait(5);
    // Now we need to write title and description
    // we will reuse from previous video 
    do_action(&EventType::MouseMove { x: 809., y: 314. });
    do_action(&EventType::ButtonPress(Button::Left));
    do_action(&EventType::ButtonRelease(Button::Left));
    wait(5);
    do_action(&EventType::MouseMove { x: 570., y: 400. });
    do_action(&EventType::ButtonPress(Button::Left));
    do_action( &EventType::ButtonRelease(Button::Left));

    wait(5);
    // accept and ok
    for i in 0..7{
        do_action(&EventType::KeyPress(Key::Tab));
        do_action(&EventType::KeyRelease(Key::Tab));
    }
    
    
    do_action(&EventType::KeyPress(Key::Return));
    do_action(&EventType::KeyRelease(Key::Return));

    // choose not for kids then next
    wait(2);

    for i in 0..18 {
        do_action(&EventType::KeyPress(Key::Tab));
        do_action(&EventType::KeyRelease(Key::Tab));
    }
    do_action(&EventType::KeyPress(Key::DownArrow));

    // Next
    for i in 0..10{
        do_action(&EventType::KeyPress(Key::Tab));

    }

    do_action(&EventType::KeyPress(Key::Return));
    do_action(&EventType::KeyRelease(Key::Return));

    //next
    wait(1);
    do_action(&EventType::KeyPress(Key::Return));
    do_action(&EventType::KeyRelease(Key::Return));

    //next
    wait(1);
    do_action(&EventType::KeyPress(Key::Return));
    do_action(&EventType::KeyRelease(Key::Return));


    //public
    do_action(&EventType::ButtonPress(Button::Left));
    do_action(&EventType::ButtonRelease(Button::Left));
    do_action(&EventType::KeyPress(Key::DownArrow));
    do_action(&EventType::KeyPress(Key::DownArrow));

  
    do_action(&EventType::KeyPress(Key::Return));
    wait(1);
    do_action(&EventType::MouseMove { x: 1200., y: 780. });
    do_action(&EventType::ButtonPress(Button::Left));
    do_action(&EventType::ButtonRelease(Button::Left));


    wait(2);















}

fn do_action(event_type: &EventType) {

    // make a random number from 20 to 400
    let millis = rand::thread_rng().gen_range(200 ..400);
    
    let delay = time::Duration::from_millis(millis);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS)
    thread::sleep(delay);
}

fn wait(sec : u64){
    let delay = time::Duration::from_secs(sec);
    std::thread::sleep(delay);
}