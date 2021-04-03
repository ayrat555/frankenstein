use crate::api_params::GetUpdatesParams;
use crate::objects::Update;

pub struct API {
    api_key: String,
}

impl API {
    pub fn get_updates(&self, params: GetUpdatesParams) -> Update {}
}
