use frankenstein::api_params::{GetUpdatesParams, SendMessageParams};
use frankenstein::client_ureq::Bot;
use frankenstein::objects::ReplyParameters;
use frankenstein::updates::UpdateContent;
use frankenstein::TelegramApi;

fn main() {
    let token = std::env::var("BOT_TOKEN").expect("Should have BOT_TOKEN as environment variable");

    let bot = Bot::new(&token);

    let mut update_params = GetUpdatesParams::builder().build();

    loop {
        let result = bot.get_updates(&update_params);
        dbg!(&result);

        match result {
            Ok(response) => {
                for update in response.result {
                    if let UpdateContent::Message(message) = update.content {
                        let reply_parameters = ReplyParameters::builder()
                            .message_id(message.message_id)
                            .build();
                        let send_message_params = SendMessageParams::builder()
                            .chat_id(message.chat.id)
                            .text("hello")
                            .reply_parameters(reply_parameters)
                            .build();
                        if let Err(error) = bot.send_message(&send_message_params) {
                            println!("Failed to send message: {error:?}");
                        }
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
