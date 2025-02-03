use std::time::Duration;

use frankenstein::client_reqwest::Bot;
use frankenstein::AsyncTelegramApi;

static TOKEN: &str = "API_TOKEN";
static BASE_API_URL: &str = "https://api.telegram.org/bot";

#[tokio::main]
async fn main() {
    let bot = custom_client();

    match bot.get_me().await {
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

fn custom_client() -> Bot {
    let client = frankenstein::reqwest::ClientBuilder::new()
        .connect_timeout(Duration::from_secs(100))
        .timeout(Duration::from_secs(100))
        .build()
        .unwrap();
    let api_url = format!("{BASE_API_URL}{TOKEN}");

    Bot::builder().api_url(api_url).client(client).build()
}
