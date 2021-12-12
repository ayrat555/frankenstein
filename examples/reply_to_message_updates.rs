use frankenstein::Api;
use frankenstein::ChatId;
use frankenstein::GetUpdatesParamsBuilder;
use frankenstein::SendMessageParamsBuilder;
use frankenstein::TelegramApi;

static TOKEN: &str = "API_TOKEN";

fn main() {
    let api = Api::new(TOKEN);

    let mut update_params_builder = GetUpdatesParamsBuilder::default();
    update_params_builder.allowed_updates(vec!["message".to_string()]);

    let mut update_params = update_params_builder.build().unwrap();

    loop {
        let result = api.get_updates(&update_params);

        println!("result: {:?}", result);

        match result {
            Ok(response) => {
                for update in response.result {
                    if let Some(message) = update.message {
                        let send_message_params = SendMessageParamsBuilder::default()
                            .chat_id(ChatId::Integer(message.chat.id))
                            .text("hello")
                            .reply_to_message_id(message.message_id)
                            .build()
                            .unwrap();

                        if let Err(err) = api.send_message(&send_message_params) {
                            println!("Failed to send message: {:?}", err);
                        }

                        update_params = update_params_builder
                            .offset(update.update_id + 1)
                            .build()
                            .unwrap();
                    }
                }
            }
            Err(error) => {
                println!("Failed to get updates: {:?}", error);
            }
        }
    }
}
