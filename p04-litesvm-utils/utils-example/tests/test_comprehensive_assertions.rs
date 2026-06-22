use litesvm_utils::{AssertionHelpers, LiteSVM, Signer, TestHelpers, TransactionHelpers};
use solana_sdk::{native_token::LAMPORTS_PER_SOL};
use solana_system_interface::instruction;

#[test]
fn test_comprehensive_assertions() {
    let mut svm = LiteSVM::new();

    // Setup
    let admin = svm.create_funded_account(100 * LAMPORTS_PER_SOL).unwrap();
    let alice = svm.create_funded_account(10 * LAMPORTS_PER_SOL).unwrap();
    let bob = svm.create_funded_account(0).unwrap();

    // Verify initial SOL balances
    svm.assert_sol_balance(&admin.pubkey(), 100 * LAMPORTS_PER_SOL);
    svm.assert_sol_balance(&alice.pubkey(), 10 * LAMPORTS_PER_SOL);
    svm.assert_sol_balance(&bob.pubkey(), 0);

    // Transfer SOL from Alice to Bob
    let transfer_ix = instruction::transfer(
        &alice.pubkey(),
        &bob.pubkey(),
        5 * LAMPORTS_PER_SOL,
    );
    svm.send_instruction(transfer_ix, &[&alice]).unwrap().assert_success();

    // Verify balances after transfer (Alice paid fees too)
    svm.assert_sol_balance(&bob.pubkey(), 5 * LAMPORTS_PER_SOL);

    // Create token infrastructure (mint returns Keypair, ATAs return Pubkey)
    let mint = svm.create_token_mint(&admin, 6).unwrap();
    let alice_ata = svm.create_associated_token_account(&mint.pubkey(), &alice).unwrap();
    let bob_ata = svm.create_associated_token_account(&mint.pubkey(), &bob).unwrap();

    // Verify account properties
    svm.assert_account_exists(&mint.pubkey());
    svm.assert_account_exists(&alice_ata);
    svm.assert_account_owner(&mint.pubkey(), &spl_token::id());
    svm.assert_account_data_len(&mint.pubkey(), 82);
    svm.assert_account_data_len(&alice_ata, 165);

    // Mint and verify
    svm.mint_to(&mint.pubkey(), &alice_ata, &admin, 1_000_000).unwrap();
    svm.mint_to(&mint.pubkey(), &bob_ata, &admin, 500_000).unwrap();

    svm.assert_token_balance(&alice_ata, 1_000_000);
    svm.assert_token_balance(&bob_ata, 500_000);
    svm.assert_mint_supply(&mint.pubkey(), 1_500_000);

    println!("All assertions passed!");
}