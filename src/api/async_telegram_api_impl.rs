use crate::api_impl::Api;
use crate::api_impl::Error;
use crate::async_api::AsyncTelegramApi;
use async_trait::async_trait;

#[async_trait]
impl AsyncTelegramApi for Api {
    type Error = Error;

    async fn request<
        T1: serde::ser::Serialize + std::fmt::Debug,
        T2: serde::de::DeserializeOwned,
    >(
        &self,
        method: &str,
        params: Option<T1>,
    ) -> Result<T2, Self::Error> {
        let url = format!("{}/{}", self.api_url, method);
    }
}
