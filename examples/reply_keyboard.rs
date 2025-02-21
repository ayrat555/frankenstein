use frankenstein::api_params::{ReplyMarkup, SendMessageParams};
use frankenstein::client_ureq::Bot;
use frankenstein::objects::{KeyboardButton, ReplyKeyboardMarkup};
use frankenstein::TelegramApi;

fn main() {
    let token = std::env::var("BOT_TOKEN").expect("Should have BOT_TOKEN as environment variable");
    let chat_id = std::env::var("TARGET_CHAT")
        .expect("Should have TARGET_CHAT as environment variable")
        .parse::<i64>()
        .expect("TARGET_CHAT should be i64");

    let bot = Bot::new(&token);

    let mut keyboard: Vec<Vec<KeyboardButton>> = Vec::new();

    for i in 1..5 {
        let mut row: Vec<KeyboardButton> = Vec::new();

        for j in 1..5 {
            let name = format!("{i}{j}");
            let button = KeyboardButton::builder().text(name).build();
            row.push(button);
        }

        keyboard.push(row);
    }

    let keyboard_markup = ReplyKeyboardMarkup::builder().keyboard(keyboard).build();
    let send_message_params = SendMessageParams::builder()
        .chat_id(chat_id)
        .text("hello!")
        .reply_markup(ReplyMarkup::ReplyKeyboardMarkup(keyboard_markup))
        .build();
    bot.send_message(&send_message_params).unwrap();
}
