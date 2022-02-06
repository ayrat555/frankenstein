use frankenstein::AsyncApi;
use frankenstein::AsyncTelegramApi;
use frankenstein::GetUpdatesParamsBuilder;
use frankenstein::Message;
use frankenstein::SendMessageParamsBuilder;

static TOKEN: &str = "API_TOKEN";

#[tokio::main]
async fn main() {
    let api = AsyncApi::new(TOKEN);

    let mut update_params_builder = GetUpdatesParamsBuilder::default();
    update_params_builder.allowed_updates(vec!["message".to_string()]);

    let mut update_params = update_params_builder.build().unwrap();

    loop {
        let result = api.get_updates(&update_params).await;

        println!("result: {:?}", result);

        match result {
            Ok(response) => {
                for update in response.result {
                    if let Some(message) = update.message {
                        let api_clone = api.clone();

                        tokio::spawn(async move {
                            process_message(message, api_clone).await;
                        });

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

async fn process_message(message: Message, api: AsyncApi) {
    let send_message_params = SendMessageParamsBuilder::default()
        .chat_id(message.chat.id)
        .text("hello")
        .reply_to_message_id(message.message_id)
        .build()
        .unwrap();

    if let Err(err) = api.send_message(&send_message_params).await {
        println!("Failed to send message: {:?}", err);
    }
}
