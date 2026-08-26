use url::Url;

use crate::{constants::BASE_URL, http::errors::HttpError};

pub enum Endpoints {
    Login,
}

impl Endpoints {
    pub fn path(&self) -> &str {
        match self {
            Self::Login => "v1/auth/login",
        }
    }

    pub fn require_auth(&self) -> bool {
        !matches!(self, Self::Login)
    }

    pub fn url(&self) -> Result<Url, HttpError> {
        BASE_URL.join(self.path()).map_err(HttpError::UrlParsing)
    }
}
