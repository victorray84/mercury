//! # Error
//!
//! Custom Error types for our crate

use shared_lib::util::UtilError;

use rocket::http::{ Status, ContentType };
use rocket::Response;
use rocket::Request;
use rocket::response::Responder;
use std::error;
use std::fmt;
use std::io::Cursor;

/// State Entity library specific errors
#[derive(Debug, Deserialize)]
pub enum SEError {
    /// Generic error from string error message
    Generic(String),
    /// Athorisation failed
    AuthError,
    /// None or incorrect sig hash found for statechain
    SigningError(String),
    /// Inherit error from Util
    Util(String)
}

impl From<String> for SEError {
    fn from(e: String) -> SEError {
        SEError::Generic(e)
    }
}

impl From<UtilError> for SEError {
    fn from(e: UtilError) -> SEError {
        SEError::Util(e.to_string())
    }
}

impl fmt::Display for SEError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SEError::Generic(ref e) => write!(f, "Error: {}", e),
            SEError::AuthError => write!(f,"User authorisation failed"),
            SEError::SigningError(ref e) => write!(f,"Signing Error: {}",e),
            SEError::Util(ref e) => write!(f,"Util Error: {}",e),
        }
    }
}

impl error::Error for SEError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            _ => None,
        }
    }
}

impl Responder<'static> for SEError {
    fn respond_to(self, _: &Request) -> ::std::result::Result<Response<'static>, Status> {
        Response::build()
            .header(ContentType::JSON)
            .sized_body(Cursor::new(format!("{}", self)))
            .ok()
    }
}
