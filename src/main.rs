use frankenstein::api::API;
use frankenstein::api_params::ChatIdEnum;
use frankenstein::api_params::GetUpdatesParams;
use frankenstein::api_params::SendMessageParams;

fn main() {
    let mut params = GetUpdatesParams::new();
    params.set_allowed_updates(Some(vec!["message".to_string()]));

    let api = API::new("1".to_string());

    let response = api.get_updates(params);

    println!("{:?}", response);

    let mut params1 =
        SendMessageParams::new(ChatIdEnum::IsizeVariant(275808073), "Hello!".to_string());

    println!("{:?}", params1);

    let response2 = api.send_message(params1);

    println!("{:?}", response2);
}
