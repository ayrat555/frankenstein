use frankenstein::reqwest;
use frankenstein::AsyncApi;
use frankenstein::AsyncTelegramApi;
use std::time::Duration;

static TOKEN: &str = "API_TOKEN";
static BASE_API_URL: &str = "https://api.telegram.org/bot";

#[tokio::main]
async fn main() {
    let api = custom_client();

    match api.get_me().await {
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

fn custom_client() -> AsyncApi {
    let client = reqwest::ClientBuilder::new()
        .connect_timeout(Duration::from_secs(100))
        .timeout(Duration::from_secs(100))
        .build()
        .unwrap();
    let api_url = format!("{BASE_API_URL}{TOKEN}");

    AsyncApi::builder().api_url(api_url).client(client).build()
}
