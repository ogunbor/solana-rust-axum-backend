use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;
use std::env;
use std::str::FromStr;

pub fn get_pubkey() -> Pubkey {
    Pubkey::from_str(
        env::var("MY_PUB_KEY")
            .expect("Error finding the public key")
            .as_str(),
    )
    .expect("Error getting the public key")
}

pub fn get_keypair() -> Keypair {
    let secret_key = env::var("MY_SECRET_KEY").expect("Error finding secret key");

    Keypair::from_base58_string(&secret_key)
}

pub fn get_client() -> RpcClient {
    let rpc_url = String::from("https://api.devnet.solana.com"); // JSON RPC URL for devnet
    RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed())
}

pub fn prepare_instruction(
    from_pubkey: &Pubkey,
    to_pubkey: &Pubkey,
    lamports_to_send: u64,
) -> Instruction {
    system_instruction::transfer(from_pubkey, to_pubkey, lamports_to_send)
}

pub fn prepare_transaction(
    ix: Instruction,
    payer: &Pubkey,
    keypair: Keypair,
    client: &RpcClient,
) -> Transaction {
    let recent_blockhash = client
        .get_latest_blockhash()
        .expect("Failed to get latest blockhash.");

    //Putting the transfer sol instruction into a transaction
    let txn = Transaction::new_signed_with_payer(&[ix], Some(payer), &[&keypair], recent_blockhash);

    txn
}