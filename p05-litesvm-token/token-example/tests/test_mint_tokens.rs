use litesvm::LiteSVM;
use litesvm_token::{
    get_spl_account,
    spl_token::{native_mint::DECIMALS, state::Account as TokenAccount},
    CreateAssociatedTokenAccount, CreateMint, MintTo,
};
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL,
    signature::{Keypair, Signer},
};

#[test]
fn test_mint_tokens() {
    let mut svm = LiteSVM::new();

    // Create payer account and fund it
    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 10 * LAMPORTS_PER_SOL).unwrap();

    // Create two users
    let alice = Keypair::new();

    // Create a new SPL token mint with alice as the mint authority
    let mint = CreateMint::new(&mut svm, &payer)
        .authority(&alice.pubkey())
        .decimals(DECIMALS)
        .send()
        .unwrap();

    // Create associated token accounts (ATAs) for alice and bob
    let alice_token_account = CreateAssociatedTokenAccount::new(&mut svm, &payer, &mint)
        .owner(&alice.pubkey())
        .send()
        .unwrap();

    // Mint 1000 tokens to Alice's account
    MintTo::new(&mut svm, &payer, &mint, &alice_token_account, 1000)
        .owner(&alice)
        .send()
        .unwrap();

    // Verify balance
    let alice_account: TokenAccount = get_spl_account(&svm, &alice_token_account).unwrap();
    let alice_balance = alice_account.amount;
    assert_eq!(alice_balance, 1000);
}