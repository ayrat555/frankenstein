use frankenstein::Api;
use frankenstein::ChatIdEnum;
use frankenstein::GetUpdatesParams;
use frankenstein::SendMessageParams;
use frankenstein::TelegramApi;

static TOKEN: &str = "API_TOKEN";

fn main() {
    let api = Api::new(TOKEN.to_string());

    let mut update_params = GetUpdatesParams::new();
    update_params.set_allowed_updates(Some(vec!["message".to_string()]));

    loop {
        let result = api.get_updates(&update_params);

        println!("result: {:?}", result);

        match result {
            Ok(response) => {
                for update in response.result {
                    if let Some(message) = update.message() {
                        let mut send_message_params = SendMessageParams::new(
                            ChatIdEnum::Integer(message.chat().id()),
                            "hello".to_string(),
                        );
                        send_message_params.set_reply_to_message_id(Some(message.message_id()));

                        let _ = api.send_message(&send_message_params);

                        update_params.set_offset(Some(update.update_id() + 1))
                    }
                }
            }
            Err(error) => {
                println!("Failed to get updates: {:?}", error);
            }
        }
    }
}
