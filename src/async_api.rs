use crate::api::MethodResponse;
use crate::api_params::GetUpdatesParams;
use crate::objects::Update;
use async_trait::async_trait;

#[async_trait]
pub trait AsyncTelegramApi {
    type Error;

    async fn get_updates(
        &self,
        params: &GetUpdatesParams,
    ) -> Result<MethodResponse<Vec<Update>>, Self::Error> {
        self.request("getUpdates", Some(params)).await
    }

    async fn request<T1: serde::ser::Serialize + std::fmt::Debug, T2: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: Option<T1>,
    ) -> Result<T2, Self::Error>;
}
