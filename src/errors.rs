use std::fmt;
use std::fmt::Display;

use serde::Deserialize;
use serde::Serialize;

/// Result type with custom error
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    kind: ErrorKind,
    msg: String,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ErrorKind {
    AlreadyExists,
    Internal,
    NotFound,
}

impl fmt::Display for Error {
    /// Formats service error for logging purposes.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl Error {
    #[must_use]
    pub fn new(kind: ErrorKind) -> Error {
        Error {
            kind,
            msg: String::new(),
        }
    }

    pub fn with_msg(kind: ErrorKind, msg: impl Display) -> Error {
        Error {
            kind,
            msg: msg.to_string(),
        }
    }

    pub fn internal(msg: impl Display) -> Error {
        Error::with_msg(ErrorKind::Internal, msg)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        tracing::error!("Database error: {:?}", e);
        match e {
            diesel::result::Error::NotFound => {
                return Self::new(ErrorKind::NotFound);
            }
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            ) => {
                return Self::new(ErrorKind::AlreadyExists);
            }
            _ => (),
        }
        Self::internal(e)
    }
}

impl From<actix_web::error::BlockingError> for Error {
    fn from(_error: actix_web::error::BlockingError) -> Self {
        Error::internal("Blocking error")
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        let kind = match e.kind() {
            std::io::ErrorKind::NotFound => ErrorKind::NotFound,
            std::io::ErrorKind::AlreadyExists => ErrorKind::AlreadyExists,
            _ => ErrorKind::Internal,
        };
        Self::with_msg(kind, e)
    }
}

impl actix_web::ResponseError for Error {
    fn error_response(&self) -> actix_web::HttpResponse {
        let mut builder = match self.kind {
            ErrorKind::NotFound => actix_web::HttpResponse::NotFound(),
            ErrorKind::AlreadyExists => actix_web::HttpResponse::Conflict(),
            // in testing builds we want to return the reason of the internal
            // error, otherwise see comment below
            #[cfg(feature = "testing")]
            ErrorKind::Internal => actix_web::HttpResponse::InternalServerError(),
            #[cfg(not(feature = "testing"))]
            ErrorKind::Internal => {
                // Internal error messages are never exposed, which are only
                // used for internal logging. This is because internal errors
                // are not expected (whereas 4xx type errors are used for errors
                // in the request), so if they occur it is because of errors in
                // the Enhance estate, which we don't want to expose lest we
                // expose a vulnerability.
                return actix_web::HttpResponse::InternalServerError().json(HttpErrorResponse {
                    code: ErrorKind::Internal,
                    message: Some("An unknown error occurred, please try again later".to_owned()),
                });
            }
        };

        builder.json(HttpErrorResponse {
            code: self.kind,
            message: Some(self.msg.clone()),
        })
    }
}

/// Serialized error responses.
/// See `ErrorKind` for the list of possible error codes.
#[derive(Serialize)]
pub(crate) struct HttpErrorResponse {
    code: ErrorKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}
