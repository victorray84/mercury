

// deposit() messages:
// 1. 2P-ECDSA to gen shared key P
// 2. user sends funding tx outpoint, B1, C1
// 3. Co-op sign kick-off tx (generated by SE)
// 3. Co-op sign back-up tx (generated by user)

use crate::ClientShim;

use super::super::utilities::requests;

const PATH_PRE: &str = "deposit";

pub fn deposit(client_shim: &ClientShim) ->(String, String) {
    let resp: (String, String) =
        requests::post(client_shim, &format!("{}/first", PATH_PRE)).unwrap();
    return resp;
}
