#[cfg(test)]
mod tests {
    extern crate server_lib;
    extern crate client_lib;
    extern crate shared_lib;
    extern crate bitcoin;

    use client_lib::*;
    use client_lib::wallet::wallet::Wallet;
    use server_lib::server;
    use shared_lib::structs::PrepareSignTxMessage;

    use bitcoin::{ Amount, TxIn, Transaction, OutPoint };
    use bitcoin::hashes::sha256d;
    use std::{thread, time};

    pub const TEST_WALLET_FILENAME: &str = "../client/test-assets/wallet.data";

    #[test]
    fn test_session_init() {
        spawn_server();
        let mut wallet = load_wallet();
        let res = client_lib::state_entity::deposit::session_init(&mut wallet);
        assert!(res.is_ok());
        println!("ID: {}",res.unwrap());
    }

    #[test]
    fn test_failed_auth() {
        spawn_server();
        let client_shim = ClientShim::new("http://localhost:8000".to_string(), None);
        if let Err(e) = ecdsa::get_master_key(&"Invalid id".to_string(), &client_shim) {
            assert_eq!(e.to_string(),"State Entity Error: User authorisation failed".to_string());
        }
    }

    //TODO: UPDATE TEST - ecdsa::sign now only works on Transactions
    // #[test]
    // fn test_ecdsa() {
    //     spawn_server();
    //
    //     let mut wallet = load_wallet();
    //     let id = client_lib::state_entity::deposit::session_init(&mut wallet).unwrap();
    //     let ps: ecdsa::PrivateShare = ecdsa::get_master_key(&id, &wallet.client_shim).unwrap();
    //
    //     for y in 0..10 {
    //         let x_pos = BigInt::from(0);
    //         let y_pos = BigInt::from(y);
    //         println!("Deriving child_master_key at [x: {}, y:{}]", x_pos, y_pos);
    //
    //         let child_master_key = ps
    //             .master_key
    //             .get_child(vec![x_pos.clone(), y_pos.clone()]);
    //
    //         let msg: BigInt = BigInt::from(12345);  // arbitrary message
    //         let signature =
    //             ecdsa::sign(&wallet.client_shim, msg, &child_master_key, x_pos, y_pos, &ps.id)
    //                 .expect("ECDSA signature failed");
    //
    //         println!(
    //             "signature = (r: {}, s: {})",
    //             signature.r.to_hex(),
    //             signature.s.to_hex()
    //         );
    //     }
    // }

    #[test]
    fn test_schnorr() {
        spawn_server();

        let client_shim = ClientShim::new("http://localhost:8000".to_string(), None);

        let share: schnorr::Share = schnorr::generate_key(&client_shim).unwrap();

        let msg: BigInt = BigInt::from(1234);  // arbitrary message
        let signature = schnorr::sign(&client_shim, msg, &share)
            .expect("Schnorr signature failed");

        println!(
            "signature = (e: {:?}, s: {:?})",
            signature.e,
            signature.s
        );
    }

    fn run_deposit(wallet: &mut Wallet) -> (String, String, Transaction, Transaction, PrepareSignTxMessage)  {
        // make TxIns for funding transaction
        let amount = Amount::ONE_BTC;
        let inputs =  vec![
        TxIn {
            previous_output: OutPoint { txid: sha256d::Hash::default(), vout: 0 },
            sequence: 0xffffffff - 2,
            witness: Vec::new(),
            script_sig: bitcoin::Script::default(),
        }
        ];
        // This addr should correspond to UTXOs being spent
        let funding_spend_addrs = vec!(wallet.get_new_bitcoin_address().unwrap());
        let resp = state_entity::deposit::deposit(
            wallet,
            inputs,
            funding_spend_addrs,
            amount
        ).unwrap();

        return resp

    }
    #[test]
    fn test_deposit() {
        spawn_server();
        let mut wallet = gen_wallet();

        let desposit = run_deposit(&mut wallet);
        println!("Shared wallet id: {:?} ",desposit.0);
        println!("Funding transaction: {:?} ",desposit.1);
        println!("Back up transaction: {:?} ",desposit.2);
    }

    #[test]
    fn test_get_statechain() {
        spawn_server();
        let mut wallet = gen_wallet();

        if let Err(e) = state_entity::api::get_statechain(&mut wallet, &String::from("id")) {
            assert!(e.to_string().contains(&String::from("No data for such identifier: StateChain id")))
        }

        let deposit = run_deposit(&mut wallet);

        let state_chain = state_entity::api::get_statechain(&mut wallet, &String::from(deposit.1.clone())).unwrap();
        assert_eq!(state_chain, vec!(deposit.0));
    }

    #[test]
    fn test_transfer() {
        spawn_server();
        let mut wallet_sender = gen_wallet();
        // deposit
        let amount = Amount::ONE_BTC;
        let inputs =  vec![
            TxIn {
                previous_output: OutPoint { txid: sha256d::Hash::default(), vout: 0 },
                sequence: 0xFFFFFFFF,
                witness: Vec::new(),
                script_sig: bitcoin::Script::default(),
            }
        ];
        // This addr should correspond to UTXOs being spent
        let funding_spend_addrs = vec!(wallet_sender.get_new_bitcoin_address().unwrap());
        let deposit_resp = state_entity::deposit::deposit(&mut wallet_sender, inputs, funding_spend_addrs, amount).unwrap();
        println!("Shared wallet id: {:?} ",deposit_resp.0);
        println!("state chain id: {:?} ",deposit_resp.1);
        println!("Funding transaction: {:?} ",deposit_resp.2);
        println!("Back up transaction: {:?} ",deposit_resp.3);
        println!("tx_b_prepare_sign_msg: {:?} ",deposit_resp.4);

        let state_chain = state_entity::api::get_statechain(&mut wallet_sender, &deposit_resp.1).unwrap();
        assert_eq!(state_chain.len(),1);

        let mut wallet_receiver = gen_wallet();
        let receiver_addr = wallet_receiver.get_new_state_entity_address().unwrap();

        let tranfer_sender_resp =
            state_entity::transfer::transfer_sender(
                &mut wallet_sender,
                &deposit_resp.0,    // shared wallet id
                &deposit_resp.1,    // state chain id
                &receiver_addr,
                deposit_resp.4     // backup tx prepare sign msg
        ).unwrap();

        println!("tranfer_sender_resp: {:?} ",tranfer_sender_resp);

        let transfer_receiver_resp  =
            state_entity::transfer::transfer_receiver(
                &mut wallet_receiver,
                &tranfer_sender_resp,
                &receiver_addr
            ).unwrap();

        println!("transfer_receiver_resp: {:?} ",transfer_receiver_resp);

        // check shared wallets have the same master public key
        assert_eq!(
            wallet_sender.get_shared_wallet(&deposit_resp.0).unwrap().private_share.master_key.public.q,
            wallet_receiver.get_shared_wallet(&transfer_receiver_resp.new_shared_wallet_id).unwrap().private_share.master_key.public.q
        );

        // check state chain is updated
        let state_chain = state_entity::api::get_statechain(&mut wallet_sender, &deposit_resp.1).unwrap();
        assert_eq!(state_chain.len(),2);
        assert_eq!(state_chain.last().unwrap().to_string(), receiver_addr.proof_key.to_string());
    }

    #[test]
    fn test_wallet_load_with_shared_wallet() {
        spawn_server();

        let mut wallet = load_wallet();
        let id = client_lib::state_entity::deposit::session_init(&mut wallet).unwrap();
        wallet.gen_shared_wallet(&id.to_string()).unwrap();

        let wallet_json = wallet.to_json();
        let wallet_rebuilt = wallet::wallet::Wallet::from_json(wallet_json, &"regtest".to_string(), ClientShim::new("http://localhost:8000".to_string(), None)).unwrap();

        let shared = wallet.shared_wallets.get(0).unwrap();
        let shared_rebuilt = wallet_rebuilt.shared_wallets.get(0).unwrap();

        assert_eq!(shared.id,shared_rebuilt.id);
        assert_eq!(shared.network,shared_rebuilt.network);
        assert_eq!(shared.private_share.id, shared_rebuilt.private_share.id);
        assert_eq!(shared.private_share.master_key.public, shared_rebuilt.private_share.master_key.public);
        assert_eq!(shared.last_derived_pos,shared_rebuilt.last_derived_pos);
    }


    fn spawn_server() {
        // Rocket server is blocking, so we spawn a new thread.
        thread::spawn(move || {
            server::get_server().launch();
        });

        let five_seconds = time::Duration::from_millis(5000);
        thread::sleep(five_seconds);
    }
    fn gen_wallet() -> Wallet {
        Wallet::new(
            &[0xcd; 32],
            &"regtest".to_string(),
            ClientShim::new("http://localhost:8000".to_string(), None)
        )
    }
    fn load_wallet() -> Wallet {
        Wallet::load_from(TEST_WALLET_FILENAME,&"regtest".to_string(),ClientShim::new("http://localhost:8000".to_string(), None)).unwrap()
    }
}
