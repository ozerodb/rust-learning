use base64::{engine::general_purpose, Engine as _};
use image::{DynamicImage, ImageOutputFormat};
use std::{io::Cursor, str::FromStr};

pub fn base64_png(img: DynamicImage) -> String {
    let mut image_data: Vec<u8> = Vec::new();

    img.write_to(&mut Cursor::new(&mut image_data), ImageOutputFormat::Png)
        .unwrap();

    general_purpose::STANDARD_NO_PAD.encode(image_data)
}

fn main() {
    dotenv::dotenv().ok();

    // Load the API key from the environment variable OCR_API_KEY
    let ocr_api_key: String = dotenv::var("OCR_API_KEY").unwrap();

    // Load the image from the file path passed as argument
    let img = image::open(std::env::args().nth(1).unwrap()).unwrap();

    // Encode the image as a base64 string
    let img_base64 = base64_png(img);
    let img_base64 = format!("data:image/png;base64,{img_base64}==");

    // Build the request body using reqwest
    let client = reqwest::blocking::Client::new();
    let res = client
        .post("https://api.ocr.space/parse/image")
        .header("apikey", &ocr_api_key)
        .form(&[
            ("base64Image", &img_base64),
            ("language", &String::from_str("eng").unwrap()),
        ])
        .send()
        .unwrap();

    // Print the response body
    let json_value = res.json::<serde_json::Value>().unwrap();
    let parsed_text: Option<&str> = json_value["ParsedResults"][0]["ParsedText"].as_str();
    println!("{}", parsed_text.unwrap());
}
