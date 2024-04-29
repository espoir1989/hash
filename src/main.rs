use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
    system_instruction,
};
use solana_sdk::signer::keypair::read_keypair_file;
use std::{str::FromStr, sync::Arc, thread};
use std::path::PathBuf;

fn main() {
    let rpc_url = String::from("https://api.testnet.solana.com");
    let client = RpcClient::new(&rpc_url);
    let client = Arc::new(client);

    // Load keypairs for two payer accounts
    let keypair_path1 = PathBuf::from("id1.json");
    let keypair_path2 = PathBuf::from("id2.json");
    let payer1 = read_keypair_file(&keypair_path1).expect("Failed to read keypair from file 1");
    let payer2 = read_keypair_file(&keypair_path2).expect("Failed to read keypair from file 2");

    let key_bytes1 = payer1.to_bytes();
    let key_bytes2 = payer2.to_bytes();

    // Use two different sets of threads for each payer
    let handles1: Vec<_> = (0..10).map(|_| {
        let client = Arc::clone(&client);
        let key_bytes = key_bytes1.clone();

        thread::spawn(move || {
            let payer = Keypair::from_bytes(&key_bytes).expect("Failed to deserialize keypair");
            send_transaction(&client, &payer);
        })
    }).collect();

    let handles2: Vec<_> = (0..10).map(|_| {
        let client = Arc::clone(&client);
        let key_bytes = key_bytes2.clone();

        thread::spawn(move || {
            let payer = Keypair::from_bytes(&key_bytes).expect("Failed to deserialize keypair");
            send_transaction(&client, &payer);
        })
    }).collect();

    // Wait for all threads to complete
    handles1.into_iter().chain(handles2).for_each(|handle| {
        handle.join().unwrap();
    });
}

fn send_transaction(client: &RpcClient, payer: &Keypair) {
//    let program_id = Pubkey::from_str("E3cXtz25rC1SeBVCkudZyWtzDfyDRcHvVUVuiReW2hiG").unwrap();
    let program_id = Pubkey::from_str("7R2KMCUW1GimTEiS8tp8jJrde2N66yQiJ1MEUTbaPgfq").unwrap();
    //let program_id = Pubkey::from_str("7R2KMCUW1GimTEiS8tp8jJrde2N66yQiJ1MEUTbaPgfq").unwrap();
    let result_account = Keypair::new();
    let space = 5;
    let rent_exemption = client.get_minimum_balance_for_rent_exemption(space).unwrap();

    let create_account_instruction = system_instruction::create_account(
        &payer.pubkey(),
        &result_account.pubkey(),
        rent_exemption,
        space as u64,
        &program_id,
    );

    let process_instruction_accounts = vec![
        AccountMeta::new(result_account.pubkey(), false),
    ];

    let process_instruction = Instruction::new_with_bincode(
        program_id,
        &[0],
        process_instruction_accounts,
    );

    let max_units = 1_200_000;
    let compute_budget_instruction = ComputeBudgetInstruction::set_compute_unit_limit(max_units);

    let mut transaction = Transaction::new_with_payer(
        &[compute_budget_instruction, create_account_instruction, process_instruction],
        Some(&payer.pubkey()),
    );

    let recent_blockhash = client.get_latest_blockhash().unwrap();
    transaction.sign(&[payer, &result_account], recent_blockhash);

    match client.send_and_confirm_transaction(&transaction) {
        Ok(_) => println!("Transaction sent successfully."),
        Err(e) => eprintln!("Failed to send transaction: {}", e),
    }
}
