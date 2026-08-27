use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Credential {
    username: String,
    password: String,
    #[serde(rename = "hostId", skip_serializing_if = "Option::is_none")]
    host_id: Option<u32>,
}

impl Credential {
    pub fn new(
        username: impl Into<String>,
        password: impl Into<String>,
        host_id: Option<u32>,
    ) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
            host_id,
        }
    }
}

#[derive(Deserialize)]
pub struct LoginResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "hoteId")]
    pub host_id: u32,
    #[serde(rename = "userId")]
    pub user_id: u32,
}
