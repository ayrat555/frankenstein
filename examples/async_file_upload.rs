use frankenstein::parameters::SendPhotoParams;
use frankenstein::{AsyncApi, AsyncTelegramApi};

static TOKEN: &str = "TOKEN";
static CHAT_ID: i64 = 1;

#[tokio::main]
async fn main() {
    let api = AsyncApi::new(TOKEN);

    let file = std::path::PathBuf::from("./frankenstein_logo.png");

    let params = SendPhotoParams::builder()
        .chat_id(CHAT_ID)
        .photo(file)
        .build();

    match api.send_photo(&params).await {
        Ok(response) => {
            println!("Photo was uploaded {response:?}");
        }
        Err(error) => {
            eprintln!("Failed to upload photo: {error:?}");
        }
    }
}
