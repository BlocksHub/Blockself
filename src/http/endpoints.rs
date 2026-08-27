use url::Url;

use crate::{constants::BASE_URL, http::errors::HttpError};

pub enum Endpoint {
    Login,
    Host(u32),
}

impl Endpoint {
    pub fn path(&self) -> String {
        match self {
            Self::Login => format!("v1/auth/login"),
            Self::Host(id) => format!("v1/hotes/{}", id),
        }
    }

    pub fn require_auth(&self) -> bool {
        !matches!(self, Self::Login)
    }

    pub fn url(&self) -> Result<Url, HttpError> {
        BASE_URL
            .join(self.path().as_str())
            .map_err(HttpError::UrlParsing)
    }
}
