use frankenstein::client_ureq::Bot;
use frankenstein::input_file::InputFile;
use frankenstein::methods::UploadStickerFileParams;
use frankenstein::stickers::StickerFormat;
use frankenstein::TelegramApi;

fn main() {
    let token = std::env::var("BOT_TOKEN").expect("Should have BOT_TOKEN as environment variable");
    let user_id = std::env::var("TARGET_USER")
        .expect("Should have TARGET_USER as environment variable")
        .parse::<u64>()
        .expect("TARGET_USER should be i64");

    let bot = Bot::new(&token);

    let file = std::path::PathBuf::from("./frankenstein_logo_sticker.png");

    let params = UploadStickerFileParams::builder()
        .user_id(user_id)
        .sticker(InputFile { path: file })
        .sticker_format(StickerFormat::Static)
        .build();

    match bot.upload_sticker_file(&params) {
        Ok(response) => {
            eprintln!("Successfully uploaded a sticker file - {response:?}");
        }
        Err(error) => {
            eprintln!("Failed to create a sticker {error:?}");
        }
    }
}
