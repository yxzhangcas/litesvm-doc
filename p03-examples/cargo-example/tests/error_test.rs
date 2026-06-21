use litesvm::LiteSVM;
use solana_sdk::signature::{Keypair, Signer};
use solana_system_interface::instruction;
use solana_transaction::Transaction;
use solana_transaction_error::TransactionError;

#[test]
fn test_insufficient_funds_error() {
    let mut svm = LiteSVM::new();

    let alice = Keypair::new();
    let bob = Keypair::new();

    // Give Alice only 0.5 SOL
    svm.airdrop(&alice.pubkey(), 500_000_000).unwrap();

    // Try to transfer 1 SOL (should fail)
    let transfer_ix = instruction::transfer(
        &alice.pubkey(),
        &bob.pubkey(),
        1_000_000_000, // More than Alice has
    );

    let tx = Transaction::new_signed_with_payer(
        &[transfer_ix],
        Some(&alice.pubkey()),
        &[&alice],
        svm.latest_blockhash(),
    );

    // Verify it fails with expected error
    let result = svm.send_transaction(tx);
    assert!(result.is_err());

    let err = result.unwrap_err();
    match err.err {
        TransactionError::InstructionError(0, _) => {
            println!("Got expected error: InstructionError (insufficient funds)");
        }
        _ => panic!("Got unexpected error: {:?}", err.err),
    }

    // Verify no funds were transferred
    assert_eq!(svm.get_balance(&bob.pubkey()).unwrap_or(0), 0);
}
