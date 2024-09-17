use frankenstein::api_params::{GetUpdatesParams, ReplyParameters, SendMessageParams};
use frankenstein::objects::{Message, UpdateContent};
use frankenstein::{AsyncApi, AsyncTelegramApi};

static TOKEN: &str = "API_TOKEN";

#[tokio::main]
async fn main() {
    let api = AsyncApi::new(TOKEN);

    let mut update_params = GetUpdatesParams::builder().build();

    loop {
        let result = api.get_updates(&update_params).await;

        println!("result: {result:?}");

        match result {
            Ok(response) => {
                for update in response.result {
                    if let UpdateContent::Message(message) = update.content {
                        let api_clone = api.clone();

                        tokio::spawn(async move {
                            process_message(message, api_clone).await;
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

async fn process_message(message: Message, api: AsyncApi) {
    let reply_parameters = ReplyParameters::builder()
        .message_id(message.message_id)
        .build();
    let send_message_params = SendMessageParams::builder()
        .chat_id(message.chat.id)
        .text("hello")
        .reply_parameters(reply_parameters)
        .build();
    if let Err(error) = api.send_message(&send_message_params).await {
        println!("Failed to send message: {error:?}");
    }
}
