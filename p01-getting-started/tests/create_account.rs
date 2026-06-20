use litesvm::LiteSVM;
use solana_sdk::signature::{Keypair, Signer};

#[test]
fn create_account() {
    // Create the test environment
    let mut svm = LiteSVM::new();

    // Create a test account
    let user = Keypair::new();

    // Fund the account with SOL
    svm.airdrop(&user.pubkey(), 1_000_000_000).unwrap();

    // Check the balance
    let balance = svm.get_balance(&user.pubkey()).unwrap();
    assert_eq!(balance, 1_000_000_000);

    println!("Account funded with {} SOL", balance as f64 / 1e9);
}