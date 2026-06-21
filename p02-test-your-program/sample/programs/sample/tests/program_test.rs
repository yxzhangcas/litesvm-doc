use anchor_lang::system_program;
use litesvm::LiteSVM;
use solana_sdk::{
    borsh1::try_from_slice_unchecked,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use solana_transaction::Transaction;

use sample::{self, PollAccount};

#[test]
fn test_program() {
    // Initialize the test environment
    let mut svm = LiteSVM::new();

    // Deploy your program to the test environment
    let program_id = Pubkey::from(sample::ID);
    let program_bytes = include_bytes!("../../../target/deploy/sample.so");
    let _ = svm.add_program(program_id, program_bytes);

    // Create and fund test accounts
    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();

    // 1. init poll
    let discriminator = [193, 22, 99, 197, 18, 33, 115, 117]; // from idl
    let poll_id: u64 = 10086;
    let start_time: u64 = 0;
    let end_time: u64 = 1000_000;
    let name = b"poll_10086";
    let description = b"poll_10086_description";

    let mut data: Vec<u8> = Vec::new();
    data.extend_from_slice(&discriminator);
    data.extend_from_slice(&poll_id.to_le_bytes());
    data.extend_from_slice(&start_time.to_le_bytes());
    data.extend_from_slice(&end_time.to_le_bytes());
    data.extend_from_slice(&(name.len() as u32).to_le_bytes());
    data.extend_from_slice(name);
    data.extend_from_slice(&(description.len() as u32).to_le_bytes());
    data.extend_from_slice(description);

    let poll_account =
        Pubkey::find_program_address(&[b"poll", &poll_id.to_le_bytes()], &sample::ID).0;

    // Create your instruction
    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(poll_account, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data,
    };

    // Build transaction
    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );

    // Send transaction
    let result = svm.send_transaction(tx).unwrap();

    // Check transaction succeeded
    println!("Transaction logs: {:?}", result.logs);

    println!("Program executed successfully!");

    let data = &svm.get_account(&poll_account).unwrap().data[8..];
    let account_data: PollAccount = try_from_slice_unchecked(&data).unwrap();

    println!(
        "{} {} {} {} {}",
        account_data.poll_name,
        account_data.poll_description,
        account_data.poll_option_index,
        account_data.poll_voting_start,
        account_data.poll_voting_end
    );
}
