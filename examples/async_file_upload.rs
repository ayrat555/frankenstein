use std::env;

use frankenstein::api_params::{InputFile, SendPhotoParams};
use frankenstein::client_reqwest::Bot;
use frankenstein::AsyncTelegramApi;

#[tokio::main]
async fn main() {
    let token = env::var("BOT_TOKEN").expect("Should have BOT_TOKEN as environment variable");
    let chat_id = env::var("TARGET_CHAT")
        .expect("Should have TARGET_CHAT as environment variable")
        .parse::<i64>()
        .expect("TARGET_CHAT should be i64");

    let bot = Bot::new(&token);

    let input_file = InputFile::read_tokio_fs("./frankenstein_logo.png")
        .await
        .expect("Should be able to read file");
    println!("File size: {}", input_file.bytes.len());

    let params = SendPhotoParams::builder()
        .chat_id(chat_id)
        .photo(input_file)
        .build();

    match bot.send_photo(&params).await {
        Ok(response) => {
            println!("Photo was uploaded successfully");
            dbg!(response);
        }
        Err(error) => {
            eprintln!("Failed to upload photo: {error}");
            dbg!(error);
        }
    }
}
