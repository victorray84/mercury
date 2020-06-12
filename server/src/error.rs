//! # Error
//!
//! Custom Error types for our crate

use shared_lib::error::SharedLibError;

use rocket::http::{ Status, ContentType };
use rocket::Response;
use rocket::Request;
use rocket::response::Responder;
use std::error;
use std::fmt;
use std::io::Cursor;
use monotree::Errors as MonotreeErrors;
use bitcoin::secp256k1::Error as SecpError;


/// State Entity library specific errors
#[derive(Debug, Deserialize)]
pub enum SEError {
    /// Generic error from string error message
    Generic(String),
    /// Athorisation failed
    AuthError,
    /// Error in co-signing
    SigningError(String),
    /// Storage error
    DBError(DBErrorType, String),
    /// Inherit errors from Util
    SharedLibError(String),
    /// Inherit errors from Monotree
    SMTError(String)
}

impl From<String> for SEError {
    fn from(e: String) -> SEError {
        SEError::Generic(e)
    }
}

impl From<SharedLibError> for SEError {
    fn from(e: SharedLibError) -> SEError {
        SEError::SharedLibError(e.to_string())
    }
}

impl From<MonotreeErrors> for SEError {
    fn from(e: MonotreeErrors) -> SEError {
        SEError::SMTError(e.to_string())
    }
}

impl From<SecpError> for SEError {
    fn from(e: SecpError) -> SEError {
        SEError::SigningError(e.to_string())
    }
}


/// Wallet error types
#[derive(Debug, Deserialize)]
pub enum DBErrorType {
    /// No data found for identifier
    NoDataForID
}

impl DBErrorType {
    fn as_str(&self) -> &'static str {
        match *self {
            DBErrorType::NoDataForID => "No data for such identifier.",
        }
    }
}

impl fmt::Display for SEError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SEError::Generic(ref e) => write!(f, "Error: {}", e),
            SEError::AuthError => write!(f,"Authentication Error: User authorisation failed"),
            SEError::DBError(ref e, ref value) => write!(f, "DB Error: {} (value: {})", e.as_str(), value),
            SEError::SigningError(ref e) => write!(f,"Signing Error: {}",e),
            SEError::SharedLibError(ref e) => write!(f,"SharedLibError Error: {}",e),
            SEError::SMTError(ref e) => write!(f,"SMT Error: {}",e),
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