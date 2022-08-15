//! Errors that come from the API endpoints

use std::{error::Error, fmt};

use reqwest::{blocking::Response, StatusCode};

use crate::error::Result;

pub fn map_api_error(resp: Response) -> Result<Response> {
    if let Err(source) = resp.error_for_status_ref() {
        let kind = source.status().into();
        return Err(ApiError { message: resp.text()?, source, kind }.into());
    }
    Ok(resp)
}

#[derive(Debug)]
#[non_exhaustive]
pub struct ApiError {
    pub message: String,
    pub source: reqwest::Error,
    pub kind: ApiErrorKind,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.message) }
}

impl Error for ApiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> { Some(&self.source) }
}

impl PartialEq for ApiError {
    fn eq(&self, other: &Self) -> bool { self.kind == other.kind }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ApiErrorKind {
    /// HTTP Status Code that isn't implemented yet
    UnimplementedHttpStatus(StatusCode),
    /// HTTP 400 - Bad Request
    BadRequest,
    /// HTTP 401 - I don't know you
    Unauthorized,
    /// HTTP 403 - I know you, but I don't like you
    Forbidden,
    /// HTTP 404 - Not Found
    NotFound,
    /// HTTP 409 - Conflict
    Conflict,
    /// HTTP 500 - Internal
    InternalServerError,
    /// HTTP 503 - Service Unavailable
    ServiceUnavailable,
    /// Not an HTTP status error
    Unknown,
}

impl From<Option<StatusCode>> for ApiErrorKind {
    fn from(code: Option<StatusCode>) -> Self {
        use ApiErrorKind::*;
        match code {
            Some(StatusCode::BAD_REQUEST) => BadRequest,
            Some(StatusCode::UNAUTHORIZED) => Unauthorized,
            Some(StatusCode::FORBIDDEN) => Forbidden,
            Some(StatusCode::NOT_FOUND) => NotFound,
            Some(StatusCode::CONFLICT) => Conflict,
            Some(StatusCode::INTERNAL_SERVER_ERROR) => InternalServerError,
            Some(StatusCode::SERVICE_UNAVAILABLE) => ServiceUnavailable,
            Some(code) => UnimplementedHttpStatus(code),
            None => Unknown,
        }
    }
}
