extern crate centipede;
extern crate config;
extern crate curv;
extern crate kms;
extern crate multi_party_ecdsa;
extern crate reqwest;
extern crate zk_paillier;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate log;

#[macro_use]
extern crate failure;

extern crate base64;
extern crate bitcoin;
extern crate electrumx_client;
extern crate hex;
extern crate itertools;
extern crate uuid;

extern crate shared_lib;

pub mod ecdsa;
pub mod error;
pub mod state_entity;
pub mod wallet;

mod utilities;

type Result<T> = std::result::Result<T, error::CError>;

#[derive(Debug, Clone)]
pub struct ClientShim {
    pub client: reqwest::Client,
    pub auth_token: Option<String>,
    pub endpoint: String,
}

impl ClientShim {
    pub fn new(endpoint: String, auth_token: Option<String>, certificate: Option<reqwest::Certificate>) -> Result<ClientShim> {
        let mut cb = reqwest::ClientBuilder::new();
        cb = match certificate {
            Some(c) => cb.add_root_certificate(c),
            None => cb
        };
        let client = cb.build()?;
        Ok(ClientShim {
            client,
            auth_token,
            endpoint,
        })
    }
}
