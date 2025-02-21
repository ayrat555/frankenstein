use std::time::Duration;

use frankenstein::client_ureq::Bot;
use frankenstein::TelegramApi;

static BASE_API_URL: &str = "https://api.telegram.org/bot";

fn custom_client() -> Bot {
    let token = std::env::var("BOT_TOKEN").expect("Should have BOT_TOKEN as environment variable");

    let config = frankenstein::ureq::Agent::config_builder()
        .http_status_as_error(false)
        .timeout_global(Some(Duration::from_secs(100)))
        .build();
    let request_agent = frankenstein::ureq::Agent::new_with_config(config);
    let api_url = format!("{BASE_API_URL}{token}");

    Bot::builder()
        .api_url(api_url)
        .request_agent(request_agent)
        .build()
}

fn main() {
    let bot = custom_client();

    match bot.get_me() {
        Ok(response) => {
            let user = response.result;
            println!(
                "Hello, I'm @{}, https://t.me/{}",
                user.first_name,
                user.username.expect("The bot must have a username.")
            );
        }
        Err(error) => {
            eprintln!("Failed to get me: {error:?}");
        }
    }
}
