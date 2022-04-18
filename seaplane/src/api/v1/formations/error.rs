//! Errors that come from the Formations endpoints

use std::{error::Error, fmt};

use reqwest::{blocking::Response, StatusCode};

use crate::error::Result;

pub fn map_error(
    resp: Response,
    context: Option<(FormationsErrorKind, String)>,
) -> Result<Response> {
    if let Err(source) = resp.error_for_status_ref() {
        let kind = source.status().into();
        let context = if let Some((for_kind, msg)) = context {
            if for_kind == kind {
                Some(msg)
            } else {
                None
            }
        } else {
            None
        };
        return Err(FormationsError {
            message: resp.text()?,
            context,
            source,
            kind,
        }
        .into());
    }
    Ok(resp)
}

#[derive(Debug)]
#[non_exhaustive]
pub struct FormationsError {
    pub message: String,
    pub context: Option<String>,
    pub source: reqwest::Error,
    pub kind: FormationsErrorKind,
}

impl fmt::Display for FormationsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for FormationsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

impl PartialEq for FormationsError {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
pub enum FormationsErrorKind {
    /// HTTP Status Code that isn't implemented yet
    UnimplementedHttpStatus(StatusCode),
    /// HTTP 400 - Bad Request
    InvalidRequest,
    /// HTTP 401 - I don't know you
    NotLoggedIn,
    /// HTTP 403 - I know you, but I don't like you
    NotAuthorized,
    /// HTTP 404 - Not Found
    FormationNotFound,
    /// HTTP 409 - Conflict
    NameAlreadyInUse,
    /// HTTP 500 - Internal
    InternalError,
    /// Not an HTTP status error
    Unknown,
}

impl From<Option<StatusCode>> for FormationsErrorKind {
    fn from(code: Option<StatusCode>) -> Self {
        use FormationsErrorKind::*;
        match code {
            Some(StatusCode::BAD_REQUEST) => InvalidRequest,
            Some(StatusCode::UNAUTHORIZED) => NotLoggedIn,
            Some(StatusCode::FORBIDDEN) => NotAuthorized,
            Some(StatusCode::NOT_FOUND) => FormationNotFound,
            Some(StatusCode::CONFLICT) => NameAlreadyInUse,
            Some(StatusCode::INTERNAL_SERVER_ERROR) => InternalError,
            Some(code) => UnimplementedHttpStatus(code),
            None => Unknown,
        }
    }
}
