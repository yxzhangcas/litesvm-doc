use litesvm_utils::{AssertionHelpers, LiteSVM, Signer, TestHelpers, TransactionHelpers};
use solana_sdk::{native_token::LAMPORTS_PER_SOL};
use solana_system_interface::instruction;

#[test]
fn test_with_utils() {
    let mut svm = LiteSVM::new();

    // Create funded accounts in one line
    let alice = svm.create_funded_account(10 * LAMPORTS_PER_SOL).unwrap();
    // 需要初始化一些sol，否则无法创建ata
    let bob = svm.create_funded_account(LAMPORTS_PER_SOL).unwrap();

    // Create a token mint easily
    let mint = svm.create_token_mint(&alice, 9).unwrap();

    // Create associated token accounts (returns Pubkey)
    // 创建ATA需要消耗SOL，所以初始的SOL会变化
    let alice_ata = svm.create_associated_token_account(&mint.pubkey(), &alice).unwrap();
    let bob_ata = svm.create_associated_token_account(&mint.pubkey(), &bob).unwrap();

    // Mint tokens
    svm.mint_to(&mint.pubkey(), &alice_ata, &alice, 1000).unwrap();

    // Assert balances
    svm.assert_token_balance(&alice_ata, 1000);
    svm.assert_token_balance(&bob_ata, 0);

    // 转账前后的余额变化进行比较，不使用初始金额
    let bob_balance = svm.get_balance(&bob.pubkey()).unwrap();
    // Execute a SOL transfer with rich result handling
    let transfer_ix = instruction::transfer(
        &alice.pubkey(),
        &bob.pubkey(),
        LAMPORTS_PER_SOL,
    );

    let result = svm.send_instruction(transfer_ix, &[&alice]).unwrap();
    result.assert_success();

    // Verify the transfer
    svm.assert_sol_balance(&bob.pubkey(), bob_balance + LAMPORTS_PER_SOL);
}