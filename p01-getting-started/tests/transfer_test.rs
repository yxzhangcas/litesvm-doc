use litesvm::LiteSVM;
use solana_sdk::{
    signature::{Keypair, Signer},
};
use solana_system_interface::instruction;
use solana_transaction::Transaction;

#[test]
fn test_transfer() {
    let mut svm = LiteSVM::new();

    // Create two accounts
    let alice = Keypair::new();
    let bob = Keypair::new();

    // Fund Alice
    svm.airdrop(&alice.pubkey(), 2_000_000_000).unwrap();

    // Create transfer instruction
    let transfer = instruction::transfer(
        &alice.pubkey(),
        &bob.pubkey(),
        1_000_000_000, // 1 SOL
    );

    // Build and sign transaction
    let tx = Transaction::new_signed_with_payer(
        &[transfer],
        Some(&alice.pubkey()),
        &[&alice],
        svm.latest_blockhash(),
    );

    // Send it (execution happens immediately)
    svm.send_transaction(tx).unwrap();

    // Check new balances
    assert_eq!(svm.get_balance(&bob.pubkey()).unwrap(), 1_000_000_000);
    assert!(svm.get_balance(&alice.pubkey()).unwrap() < 1_000_000_000);

    println!("Transfer successful!");
}