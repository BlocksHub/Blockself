use std::fmt;

#[derive(Debug, uniffi::Error)]
#[uniffi(flat_error)]
pub enum HttpError {
    UrlParsing(url::ParseError),
    Request(reqwest::Error),
    Serialize(serde_json::Error),
    Unauthenticated,
    PoisonedSession,
    InvalidStatus(reqwest::StatusCode, String),
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UrlParsing(e) => write!(f, "failed to parse URL: {e}"),
            Self::Request(e) => write!(f, "HTTP request failed: {e}"),
            Self::Serialize(e) => write!(f, "failed to serialize payload: {e}"),
            Self::Unauthenticated => write!(f, "this endpoint require to be logged in"),
            Self::PoisonedSession => write!(f, "session lock is poisoned"),
            Self::InvalidStatus(status, body) => {
                write!(f, "turboself api returned {status}: {body}")
            }
        }
    }
}

impl std::error::Error for HttpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::UrlParsing(e) => Some(e),
            Self::Request(e) => Some(e),
            Self::Serialize(e) => Some(e),
            _ => None,
        }
    }
}
