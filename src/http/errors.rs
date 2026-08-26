use std::fmt;

#[derive(Debug)]
pub enum HttpError {
    UrlParsing(url::ParseError),
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UrlParsing(e) => write!(f, "failed to parse URL: {e}"),
        }
    }
}

impl std::error::Error for HttpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::UrlParsing(e) => Some(e),
        }
    }
}
