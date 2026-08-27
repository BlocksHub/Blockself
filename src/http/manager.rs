use std::{
    sync::{PoisonError, RwLock, RwLockReadGuard},
    time::{Duration, SystemTime},
};

use serde::{Serialize, de::DeserializeOwned};

use crate::{
    constants::USER_AGENT,
    http::{endpoints::Endpoint, errors::HttpError},
    session::Session,
};

pub struct HttpManager {
    transport: reqwest::Client,
    session: RwLock<Session>,
}

impl HttpManager {
    pub(crate) fn transport() -> Result<reqwest::Client, HttpError> {
        reqwest::ClientBuilder::new()
            .user_agent(USER_AGENT)
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(HttpError::Request)
    }

    pub(crate) fn from_token(transport: reqwest::Client, access_token: String) -> Self {
        Self {
            transport,
            session: RwLock::new(Session {
                access_token,
                expire_at: SystemTime::now() + Duration::from_hours(5),
            }),
        }
    }

    fn session(&self) -> RwLockReadGuard<'_, Session> {
        self.session.read().unwrap_or_else(PoisonError::into_inner)
    }

    pub(crate) fn access_token(&self) -> Result<String, HttpError> {
        let session = self.session();
        if session.is_expired() {
            return Err(HttpError::Unauthenticated);
        };

        Ok(session.access_token.clone())
    }

    pub(crate) async fn post_anonymous<T, K>(
        transport: &reqwest::Client,
        endpoint: Endpoint,
        payload: Option<T>,
    ) -> Result<K, HttpError>
    where
        T: Serialize,
        K: DeserializeOwned,
    {
        let request =
            Self::build_request(transport, reqwest::Method::POST, endpoint, None, payload)?;
        Self::send::<K>(request).await
    }

    pub(crate) async fn get<K: DeserializeOwned>(
        &self,
        endpoint: Endpoint,
    ) -> Result<K, HttpError> {
        let token = if endpoint.require_auth() {
            Some(self.access_token()?)
        } else {
            None
        };
        let request = Self::build_request(
            &self.transport,
            reqwest::Method::GET,
            endpoint,
            token,
            None::<()>,
        )?;
        Self::send::<K>(request).await
    }

    async fn send<K>(request: reqwest::RequestBuilder) -> Result<K, HttpError>
    where
        K: DeserializeOwned,
    {
        request
            .send()
            .await
            .map_err(HttpError::Request)?
            .error_for_status()
            .map_err(HttpError::Request)?
            .json()
            .await
            .map_err(HttpError::Request)
    }

    pub(crate) fn build_request<T>(
        transport: &reqwest::Client,
        method: reqwest::Method,
        endpoint: Endpoint,
        token: Option<String>,
        payload: Option<T>,
    ) -> Result<reqwest::RequestBuilder, HttpError>
    where
        T: Serialize,
    {
        let mut request = transport.request(method, endpoint.url()?);
        if let Some(token) = token {
            request = request.bearer_auth(token);
        };

        if let Some(body) = payload {
            let serialized = serde_json::to_string(&body).map_err(HttpError::Serialize)?;
            request = request
                .body(serialized)
                .header("Content-Type", "application/json");
        }

        Ok(request)
    }
}
