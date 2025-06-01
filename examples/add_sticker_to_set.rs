use frankenstein::TelegramApi;
use frankenstein::client_ureq::Bot;
use frankenstein::input_file::{FileUpload, InputFile};
use frankenstein::methods::{
    AddStickerToSetParams, CreateNewStickerSetParams, UploadStickerFileParams,
};
use frankenstein::stickers::{InputSticker, StickerFormat};

fn main() {
    let token = std::env::var("BOT_TOKEN").expect("Should have BOT_TOKEN as environment variable");
    let user_id = std::env::var("TARGET_USER")
        .expect("Should have TARGET_USER as environment variable")
        .parse::<u64>()
        .expect("TARGET_USER should be i64");

    let set_name = std::env::var("STICKER_SET_NAME")
        .expect("Should have STICKER_SET_NAME as environment variable");

    let bot = Bot::new(&token);

    let file = std::path::PathBuf::from("./frankenstein_logo_sticker.png");

    let sticker = InputSticker::builder()
        .sticker(FileUpload::InputFile(InputFile { path: file }))
        .format(StickerFormat::Static)
        .emoji_list(vec!["🐰".to_string()])
        .build();

    let params = AddStickerToSetParams::builder()
        .user_id(user_id)
        .name(set_name)
        .sticker(sticker)
        .build();

    match bot.add_sticker_to_set(&params) {
        Ok(response) => {
            eprintln!("Successfully added a sticker to set - {response:?}");
        }
        Err(error) => {
            eprintln!("Failed to add a sticker set: {error:?}");
            return;
        }
    }
}
