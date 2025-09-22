use thiserror::Error;
use wreq::StatusCode;

#[derive(Error, Debug)]
pub enum YahooError {
    #[error("fetching the data from yahoo! finance failed: {0}")]
    FetchFailed(String),
    #[error("deserializing response from yahoo! finance failed: {0}")]
    DeserializeFailed(#[from] serde_json::Error),
    #[error("connection to yahoo! finance server failed: {0}")]
    ConnectionFailed(wreq::Error),
    #[error("yahoo! finance return invalid JSON format")]
    InvalidJson,
    #[error("yahoo! finance returned an empty data set")]
    EmptyDataSet,
    #[error("yahoo! finance returned inconsistent data")]
    DataInconsistency,
    #[error("construcing yahoo! finance client failed")]
    BuilderFailed,
    #[error("server reports too many requests while {0}: {1}")]
    TooManyRequests(String, #[source] wreq::Error),
    #[error("unexpected response while {0}: {1}")]
    UnexpectedResponse(String, #[source] wreq::Error),
    #[error("request didn't succeed in {0} retries")]
    MaxRetriesReached(u8, #[source] Box<YahooError>),
}

impl YahooError {
    pub fn from_wreq_while(err: wreq::Error, action: &str) -> Self {
        if err.is_connect() {
            return Self::ConnectionFailed(err);
        }
        if let Some(status) = err.status() {
            if status == StatusCode::TOO_MANY_REQUESTS {
                return Self::TooManyRequests(action.into(), err);
            }
        }

        Self::UnexpectedResponse(action.into(), err)
    }
}
