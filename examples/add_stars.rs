use frankenstein::payments::LabeledPrice;
use frankenstein::TelegramApi;
use frankenstein::client_ureq::Bot;
use frankenstein::methods::{AnswerPreCheckoutQueryParams, GetUpdatesParams, SendInvoiceParams};
use frankenstein::types::AllowedUpdate;
use frankenstein::updates::UpdateContent;

fn main() {
    // you need to send a message /start to bot
    let your_token = std::env::var("BOT_TOKEN").expect("Should have BOT_TOKEN as environment variable");
    let your_id = std::env::var("YOUR_ID").expect("Should have YOUR_ID as environment variable");

    // stars
    let amount = 10;

    let bot = Bot::new(&your_token);

    // send invoice
    let send_invoice = SendInvoiceParams::builder()
        .chat_id(your_id)
        .title("Send stars".to_string())
        .description("Get stars for bot".to_string())
        .payload("gift_purchase".to_string())
        .currency("XTR".to_string())
        .prices(vec![LabeledPrice {
            label: "amount".to_string(),
            amount: amount,
        }])
        .build();
    if let Err(error) = bot.send_invoice(&send_invoice) {
        println!("Error send_invoice: {error:?}");
    }

    // waiting for payment confirmation
    let mut update_params = GetUpdatesParams::builder()
        .allowed_updates(vec![AllowedUpdate::PreCheckoutQuery])
        .timeout(100)
        .build();

    loop {
        let result = bot.get_updates(&update_params);

        match result {
            Ok(response) => {
                for update in response.result {
                    if let UpdateContent::PreCheckoutQuery(message) = update.content {
                        let bot_clone = bot.clone();
                        let params = AnswerPreCheckoutQueryParams::builder()
                            .pre_checkout_query_id(message.id.clone())
                            .ok(true)
                            .build();
                        bot_clone.answer_pre_checkout_query(&params).ok();
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
