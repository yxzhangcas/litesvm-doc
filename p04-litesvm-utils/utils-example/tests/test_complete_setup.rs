use litesvm_utils::{AssertionHelpers, LiteSVM, Signer, TestHelpers};
use solana_sdk::native_token::LAMPORTS_PER_SOL;

#[test]
fn test_complete_setup() {
    let mut svm = LiteSVM::new();

    // Create accounts
    let admin = svm.create_funded_account(100 * LAMPORTS_PER_SOL).unwrap();
    let users = svm.create_funded_accounts(3, 10 * LAMPORTS_PER_SOL).unwrap();

    // Create token infrastructure (returns Keypair)
    let mint = svm.create_token_mint(&admin, 6).unwrap(); // 6 decimals like USDC

    // Create ATAs and mint tokens to each user
    for (i, user) in users.iter().enumerate() {
        // create_associated_token_account returns Pubkey
        let ata = svm.create_associated_token_account(&mint.pubkey(), user).unwrap();
        let amount = (i as u64 + 1) * 1_000_000; // 1, 2, 3 tokens
        svm.mint_to(&mint.pubkey(), &ata, &admin, amount).unwrap();

        // Verify
        svm.assert_token_balance(&ata, amount);
    }

    // Verify total supply
    let total_supply = 1_000_000 + 2_000_000 + 3_000_000;
    svm.assert_mint_supply(&mint.pubkey(), total_supply);

    // Test time-based logic
    let start_slot = svm.get_current_slot();
    svm.advance_slot(1000);
    assert!(svm.get_current_slot() > start_slot);
}