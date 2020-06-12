//! # Error
//!
//! Custom Error types for client

use shared_lib::error::SharedLibError;

use std::error;
use std::fmt;
use bitcoin::util::bip32::Error as Bip32Error;
use bitcoin::util::address::Error as AddressError;
use reqwest::Error as ReqwestError;
use std::num::ParseIntError;


/// Client specific errors
#[derive(Debug, Deserialize)]
pub enum CError {
    /// Generic error from string error message
    Generic(String),
    /// Wallet
    WalletError(WalletErrorType),
    /// State entity errors
    StateEntityError(String),
    /// Schnorr error
    SchnorrError(String),
    /// Inherit errors from SharedLibError
    SharedLib(String),
}

impl From<String> for CError {
    fn from(e: String) -> CError {
        CError::Generic(e)
    }
}
impl From<&str> for CError {
    fn from(e: &str) -> CError {
        CError::Generic(e.to_string())
    }
}

impl From<Bip32Error> for CError {
    fn from(e: Bip32Error) -> CError {
        CError::Generic(e.to_string())
    }
}
impl From<AddressError> for CError {
    fn from(e: AddressError) -> CError {
        CError::Generic(e.to_string())
    }
}
impl From<ReqwestError> for CError {
    fn from(e: ReqwestError) -> CError {
        CError::Generic(e.to_string())
    }
}
impl From<ParseIntError> for CError {
    fn from(e: ParseIntError) -> CError {
        CError::Generic(e.to_string())
    }
}

impl From<SharedLibError> for CError {
    fn from(e: SharedLibError) -> CError {
        CError::SharedLib(e.to_string())
    }
}
/// Wallet error types
#[derive(Debug, Deserialize)]
pub enum WalletErrorType {
    /// No key found in wallet derivaton
    KeyNotFound,
    SharedKeyNotFound,
    NotEnoughFunds
}

impl WalletErrorType {
    fn as_str(&self) -> &'static str {
        match *self {
            WalletErrorType::SharedKeyNotFound => "No shared key found.",
            WalletErrorType::NotEnoughFunds => "Not enough funds.",
            WalletErrorType::KeyNotFound => "No key found in wallet derivation path.",
        }
    }
}

impl fmt::Display for CError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CError::Generic(ref e) => write!(f, "Error: {}", e),
            CError::WalletError(ref e) => write!(f, "Wallet Error: {}", e.as_str()),
            CError::StateEntityError(ref e) => write!(f, "State Entity Error: {}", e),
            CError::SchnorrError(ref e) => write!(f, "Schnorr Error: {}", e),
            CError::SharedLib(ref e) => write!(f,"Util Error: {}",e),
        }
    }
}

impl error::Error for CError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            _ => None,
        }
    }
}