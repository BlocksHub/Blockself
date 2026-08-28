use std::sync::Arc;

use crate::{
    http::{endpoints::Endpoint, errors::HttpError, manager::HttpManager},
    models::{
        credentials::{Credential, LoginResponse},
        host::Host,
    },
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

        let http = HttpManager::from_token(transport, login.access_token);

        Ok(Arc::new(Self {
            host_id: login.host_id,
            user_id: login.user_id,
            http,
        }))
    }

    pub async fn host(&self) -> Result<Host, HttpError> {
        self.http.get(Endpoint::Host(self.host_id)).await
    }

    pub fn host_id(&self) -> u32 {
        self.host_id
    }

    pub fn user_id(&self) -> u32 {
        self.user_id
    }
}
