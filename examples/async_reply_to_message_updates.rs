use frankenstein::api_params::{GetUpdatesParams, ReplyParameters, SendMessageParams};
use frankenstein::client_reqwest::Bot;
use frankenstein::objects::{Message, UpdateContent};
use frankenstein::AsyncTelegramApi;

#[tokio::main]
async fn main() {
    let token = std::env::var("BOT_TOKEN").expect("Should have BOT_TOKEN as environment variable");

    let bot = Bot::new(&token);

    let mut update_params = GetUpdatesParams::builder().build();

    loop {
        let result = bot.get_updates(&update_params).await;
        dbg!(&result);

        match result {
            Ok(response) => {
                for update in response.result {
                    if let UpdateContent::Message(message) = update.content {
                        let bot_clone = bot.clone();

                        tokio::spawn(async move {
                            process_message(message, bot_clone).await;
                        });
                    }
                    update_params.offset = Some(i64::from(update.update_id) + 1);
                }
            }
            Err(error) => {
                println!("Failed to get updates: {error:?}");
            }
        }
    }
}

async fn process_message(message: Message, bot: Bot) {
    let reply_parameters = ReplyParameters::builder()
        .message_id(message.message_id)
        .build();
    let send_message_params = SendMessageParams::builder()
        .chat_id(message.chat.id)
        .text("hello")
        .reply_parameters(reply_parameters)
        .build();
    if let Err(error) = bot.send_message(&send_message_params).await {
        println!("Failed to send message: {error:?}");
    }
}
