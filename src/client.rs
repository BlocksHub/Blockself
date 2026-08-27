use std::sync::Arc;

use crate::{
    http::{endpoints::Endpoint, errors::HttpError, manager::HttpManager},
    models::credentials::{Credential, LoginResponse},
};

#[derive(uniffi::Object)]
pub struct Client {
    pub(crate) http: HttpManager,
    pub host_id: u32,
    pub user_id: u32,
}

#[uniffi::export(async_runtime = "tokio")]
impl Client {
    #[uniffi::constructor]
    pub async fn login(
        username: String,
        password: String,
        host_id: Option<u32>,
    ) -> Result<Arc<Self>, HttpError> {
        let transport = HttpManager::transport()?;
        let credentials = Credential::new(username, password, host_id);
        let login: LoginResponse =
            HttpManager::post_anonymous(&transport, Endpoint::Login, Some(credentials)).await?;

        Ok(Arc::new(Self {
            host_id: login.host_id,
            user_id: login.user_id,
            http: HttpManager::from_token(transport, login.access_token),
        }))
    }
}
