//! Errors produced or propagated through the Seaplane SDK

use thiserror::Error;

pub type Result<T> = std::result::Result<T, SeaplaneError>;

#[derive(Error, Debug)]
pub enum SeaplaneError {
    #[error("http request error")]
    Http(#[from] reqwest::Error),
    #[error("request did not include a required authorization token")]
    MissingRequestAuthToken,
    #[error("request did not include the required formation name")]
    MissingFormationName,
    #[error("invalid URL")]
    UrlParse(#[from] url::ParseError),
    #[error("invalid json")]
    Json(#[from] serde_json::error::Error),
    #[error("request did not include any active configurations while force=false")]
    MissingActiveConfiguration,
    #[error("missing a required UUID")]
    MissingUuid,
    #[error("the given request conflict with one another")]
    ConflictingParams,
    #[error("flights cannot be empty")]
    EmptyFlights,
    #[error("missing required Flight name")]
    MissingFlightName,
    #[error("missing required Flight image URL")]
    MissingFlightImageUrl,
    #[error("the requirements specified in the builder are in conflict and invalid")]
    ConflictingRequirements,
}
