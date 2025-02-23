use frankenstein::api_params::{InputFile, SendPhotoParams};
use frankenstein::client_ureq::Bot;
use frankenstein::TelegramApi;

fn main() {
    let token = std::env::var("BOT_TOKEN").expect("Should have BOT_TOKEN as environment variable");
    let chat_id = std::env::var("TARGET_CHAT")
        .expect("Should have TARGET_CHAT as environment variable")
        .parse::<i64>()
        .expect("TARGET_CHAT should be i64");

    let bot = Bot::new(&token);

    let file = InputFile::read_std("./frankenstein_logo.png").expect("Should be able to read file");
    println!("File size: {}", file.bytes.len());

    let params = SendPhotoParams::builder()
        .chat_id(chat_id)
        .photo(file)
        .build();
    match bot.send_photo(&params) {
        Ok(response) => {
            println!("Photo was uploaded successfully");
            dbg!(response);
        }
        Err(error) => {
            eprintln!("Failed to upload photo: {error:?}");
        }
    }
}
