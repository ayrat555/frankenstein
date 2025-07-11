use frankenstein::client_ureq::Bot;
use frankenstein::methods::{AnswerPreCheckoutQueryParams, GetUpdatesParams, SendInvoiceParams};
use frankenstein::payments::LabeledPrice;
use frankenstein::types::AllowedUpdate;
use frankenstein::updates::UpdateContent;
use frankenstein::TelegramApi;

fn main() {
    let token =
        std::env::var("BOT_TOKEN").expect("Should have BOT_TOKEN as environment variable");
    let user_id = std::env::var("TARGET_USER").expect("Should have TARGET_USER as environment variable");

    let number_of_stars = 10;

    let bot = Bot::new(&token);

    let send_invoice = SendInvoiceParams::builder()
        .chat_id(user_id)
        .title("Send stars".to_string())
        .description("Get stars for bot".to_string())
        .payload("gift_purchase".to_string())
        .currency("XTR".to_string())
        .prices(vec![LabeledPrice {
            label: "amount".to_string(),
            amount: number_of_stars,
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
