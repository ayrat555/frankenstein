use frankenstein::api::API;
use frankenstein::api_params::GetUpdatesParams;

fn main() {
    let mut params = GetUpdatesParams::new();
    params.set_allowed_updates(Some(vec!["message".to_string()]));

    let response = API::new("1".to_string()).get_updates(params);

    println!("{:?}", response);
}
