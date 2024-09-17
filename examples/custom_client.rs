use std::time::Duration;

use frankenstein::{Api, TelegramApi};

static TOKEN: &str = "API_TOKEN";
static BASE_API_URL: &str = "https://api.telegram.org/bot";

fn main() {
    let api = custom_client();

    match api.get_me() {
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

fn custom_client() -> Api {
    let request_agent = frankenstein::ureq::builder()
        .timeout(Duration::from_secs(100))
        .build();
    let api_url = format!("{BASE_API_URL}{TOKEN}");

    Api::builder()
        .api_url(api_url)
        .request_agent(request_agent)
        .build()
}
