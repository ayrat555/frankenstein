use frankenstein::GetUpdatesParams;
use frankenstein::SendMessageParams;
use frankenstein::TelegramApi;
use frankenstein::{Api, UpdateContent};

static TOKEN: &str = "API_TOKEN";

fn main() {
    let api = Api::new(TOKEN);

    let update_params_builder =
        GetUpdatesParams::builder().allowed_updates(vec!["message".to_string()]);
    let mut update_params = update_params_builder.clone().build();

    loop {
        let result = api.get_updates(&update_params);

        println!("result: {:?}", result);

        match result {
            Ok(response) => {
                for update in response.result {
                    if let UpdateContent::Message(message) = update.content {
                        let send_message_params = SendMessageParams::builder()
                            .chat_id(message.chat.id)
                            .text("hello")
                            .reply_to_message_id(message.message_id)
                            .build();

                        if let Err(err) = api.send_message(&send_message_params) {
                            println!("Failed to send message: {:?}", err);
                        }

                        update_params = update_params_builder
                            .clone()
                            .offset(update.update_id + 1)
                            .build();
                    }
                }
            }
            Err(error) => {
                println!("Failed to get updates: {:?}", error);
            }
        }
    }
}
