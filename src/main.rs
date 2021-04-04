use frankenstein::api::API;
use frankenstein::api_params::GetUpdatesParams;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut params = GetUpdatesParams::new();
    params.set_allowed_updates(Some(vec!["message".to_string()]));

    API::new("1".to_string()).get_updates(params).await?;

    Ok(())
}
