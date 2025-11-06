use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::copy;

#[derive(Debug, Deserialize)]
struct DogImage {
    message: String,
    status: String,
}

#[derive(Debug)]
enum ApiResult {
    Success(DogImage),
    ApiError(String),
    NetworkError(String),
}

fn fetch_random_dog_image() -> ApiResult {
    let url = "https://dog.ceo/api/breeds/image/random";
    
    match ureq::get(url).call() {
        Ok(response) => {
            if response.status() == 200 {
                match response.into_json::<DogImage>() {
                    Ok(dog_image) => ApiResult::Success(dog_image),
                    Err(e) => ApiResult::ApiError(format!("Failed to parse JSON: {}", e)),
                }
            } else {
                ApiResult::ApiError(format!("HTTP error: {}", response.status()))
            }
        },
        Err(e) => {
            let error_details = format!("Request failed: {}", e);
            ApiResult::NetworkError(error_details)
        },
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Dog Image Fetcher");
    println!("=================\n");

    for i in 1..=5 {
        println!("Fetching random dog image #{}", i);
        match fetch_random_dog_image() {
            ApiResult::Success(dog_image) => {
                println!("✅ Success!");

                let url = &dog_image.message;

                let filename = format!("dog_{}.jpg", i);
                
                println!("🖼️ Image URL: {}", dog_image.message);
                println!("📊 Status: {}", dog_image.status);
                
                println!("⬇️ Downloading to: **{}**", filename);

                match ureq::get(url).call() {
                    Ok(response) => {
                        if response.status() != 200 {
                            println!("❌ Download Error: HTTP status: {}", response.status());
                            println!();
                            continue;
                        }
                        
                        match File::create(&filename) {
                            Ok(mut dest) => {
                                match copy(&mut response.into_reader(), &mut dest) {
                                    Ok(bytes_copied) => {
                                        println!(" ✅ Download complete: **{} bytes written.**", bytes_copied);
                                        println!("🎉 Image saved successfully!");
                                    },
                                    Err(e) => println!("❌ Download Error: Failed to copy data: {}", e),
                                }
                            },
                            Err(e) => println!("❌ Download Error: Failed to create file: {}", e),
                        }
                    },
                    Err(e) => {
                        println!("❌ Download Error: Request failed: {}", e);
                    }
                }
            },
            ApiResult::ApiError(e) => println!("❌ API Error: {}", e),
            ApiResult::NetworkError(e) => println!("❌ Network Error: {}", e),
        }
        println!();
    }

    Ok(())
}