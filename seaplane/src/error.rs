//! Errors produced or propagated through the Seaplane SDK

use thiserror::Error;

use crate::api::ApiError;

pub type Result<T> = std::result::Result<T, SeaplaneError>;

#[derive(Error, Debug)]
pub enum SeaplaneError {
    #[error("http error")]
    UnknownHttp(reqwest::Error),
    #[error("{0}")]
    Decode(String),
    #[error("request did not include a required API key")]
    MissingRequestApiKey,
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
    #[error("missing required Flight image reference")]
    MissingFlightImageReference,
    #[error("the requirements specified in the builder are in conflict and invalid")]
    ConflictingRequirements,
    #[error("request did not include the required key")]
    MissingMetadataKey,
    #[error("request must target either key or range")]
    IncorrectMetadataRequestTarget,
    #[error("the API returned an error status")]
    ApiResponse(#[from] ApiError),
    #[error("locks requests must target either a lock by name or a held lock")]
    IncorrectLocksRequestTarget,
}

impl From<reqwest::Error> for SeaplaneError {
    fn from(re: reqwest::Error) -> Self {
        if re.is_decode() {
            SeaplaneError::Decode(re.to_string())
        } else {
            SeaplaneError::UnknownHttp(re)
        }
    }
}

impl PartialEq for SeaplaneError {
    fn eq(&self, rhs: &Self) -> bool {
        use SeaplaneError::*;

        match self {
            UnknownHttp(_) => matches!(rhs, UnknownHttp(_)),
            Decode(_) => matches!(rhs, Decode(_)),
            MissingRequestApiKey => matches!(rhs, MissingRequestApiKey),
            MissingRequestAuthToken => matches!(rhs, MissingRequestAuthToken),
            MissingFormationName => matches!(rhs, MissingFormationName),
            UrlParse(_) => matches!(rhs, UrlParse(_)),
            Json(_) => matches!(rhs, Json(_)),
            MissingActiveConfiguration => matches!(rhs, MissingActiveConfiguration),
            MissingUuid => matches!(rhs, MissingUuid),
            ConflictingParams => matches!(rhs, ConflictingParams),
            EmptyFlights => matches!(rhs, EmptyFlights),
            MissingFlightName => matches!(rhs, MissingFlightName),
            MissingFlightImageReference => matches!(rhs, MissingFlightImageReference),
            ConflictingRequirements => matches!(rhs, ConflictingRequirements),
            MissingMetadataKey => matches!(rhs, MissingMetadataKey),
            IncorrectMetadataRequestTarget => matches!(rhs, IncorrectMetadataRequestTarget),
            IncorrectLocksRequestTarget => matches!(rhs, IncorrectLocksRequestTarget),
            ApiResponse(ae) => match rhs {
                ApiResponse(oae) => ae == oae,
                _ => false,
            },
        }
    }
}
