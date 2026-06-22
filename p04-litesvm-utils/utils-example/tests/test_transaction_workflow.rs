use litesvm_utils::{AssertionHelpers, LiteSVM, Signer, TestHelpers, TransactionHelpers};
use solana_sdk::{native_token::LAMPORTS_PER_SOL};
use solana_system_interface::instruction;

#[test]
fn test_transaction_workflow() {
    let mut svm = LiteSVM::new();

    // Setup accounts
    let alice = svm.create_funded_account(10 * LAMPORTS_PER_SOL).unwrap();
    let bob = svm.create_funded_account(0).unwrap();

    // Test successful transfer
    let transfer_ix = instruction::transfer(
        &alice.pubkey(),
        &bob.pubkey(),
        2 * LAMPORTS_PER_SOL,
    );

    let result = svm.send_instruction(transfer_ix, &[&alice]).unwrap();

    // Verify success
    result.assert_success();

    // Check compute usage
    let cu = result.compute_units();
    println!("Transfer used {} compute units", cu);

    // Verify balances
    svm.assert_sol_balance(&bob.pubkey(), 2 * LAMPORTS_PER_SOL);

    // Test expected failure
    let bad_transfer = instruction::transfer(
        &alice.pubkey(),
        &bob.pubkey(),
        100 * LAMPORTS_PER_SOL, // More than Alice has
    );

    let fail_result = svm.send_instruction(bad_transfer, &[&alice]).unwrap();
    fail_result.assert_failure();

    // Print logs for debugging
    fail_result.print_logs();

    println!("Transaction workflow test passed!");
}