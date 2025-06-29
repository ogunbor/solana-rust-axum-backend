use std::str::FromStr;

use axum::Json;
use solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};

use crate::{model::transaction::TransactionSolPayload, util::basic_util};

pub async fn get_balance() -> f64 {
    let pub_key = &basic_util::get_pubkey();
    let client = basic_util::get_client();

    match client.get_balance(pub_key) {
        Ok(balance) => balance as f64 / LAMPORTS_PER_SOL as f64,
        Err(_) => {
            println!("Error getting balance");
            0.0
        }
    }
}

// airdropping sols
pub async fn get_sols() {
    let client = basic_util::get_client();

    let from_pubkey = basic_util::get_pubkey();

    // get sols (this are used for transactions)
    match client.request_airdrop(&from_pubkey, LAMPORTS_PER_SOL * 5) {
        Ok(sig) => loop {
            if let Ok(confirmed) = client.confirm_transaction(&sig) {
                if confirmed {
                    println!("Transaction: {} Status: {}", sig, confirmed);
                    break;
                }
            }
        },
        Err(e) => {
            println!("Error requesting airdrop: {}", e);
        }
    }
}

pub async fn transact_sol(payload: Json<TransactionSolPayload>) {
    let client = basic_util::get_client();

    let from_pubkey = basic_util::get_pubkey();

    // get instruction that will passed to the transaction
    let ix = basic_util::prepare_instruction(
        &from_pubkey,
        &Pubkey::from_str(&payload.to_pubkey).expect("Failed to validate receiver's account"),
        (payload.sol_to_send.parse::<f64>().unwrap() * (LAMPORTS_PER_SOL as f64)) as u64,
    );

    // get keypair from the sender's secret key
    let keypair = basic_util::get_keypair();

    // add prepared instruction to a transaction
    let txn = basic_util::prepare_transaction(ix, &from_pubkey, keypair, &client);

    // sending the transfer sol transaction
    match client.send_and_confirm_transaction(&txn) {
        Ok(sig) => loop {
            if let Ok(confirmed) = client.confirm_transaction(&sig) {
                if confirmed {
                    println!("Transaction: {} Status: {}", sig, confirmed);
                    break;
                }
            }
        },
        Err(e) => {
            println!("Error transferring Sol:, {}", e);
        }
    }
}