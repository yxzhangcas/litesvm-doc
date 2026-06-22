use litesvm::LiteSVM;
use litesvm_token::{CreateAccount, CreateAssociatedTokenAccount, CreateMint, spl_token::native_mint::DECIMALS};
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL,
    signature::{Keypair, Signer},
};

#[test]
fn test_create_mint() {
    let mut svm = LiteSVM::new();

    // Create payer account and fund it
    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 10 * LAMPORTS_PER_SOL).unwrap();

    // Create a new SPL token mint with the payer as the mint authority
    let mint = CreateMint::new(&mut svm, &payer)
        .authority(&payer.pubkey())
        .decimals(DECIMALS)
        .send()
        .unwrap();

    // Create a token account for the payer
    // You must have created a Mint Account to be able to create a Token Account
    let token_account = CreateAccount::new(&mut svm, &payer, &mint)
        .owner(&payer.pubkey())
        .send()
        .unwrap();

    // Create an ATA for the payer
    // You must have created a Mint Account to be able to create a Token Account
    let associated_token_account = CreateAssociatedTokenAccount::new(&mut svm, &payer, &mint)
        .owner(&payer.pubkey())
        .send()
        .unwrap();

    println!("{}", token_account.to_string());
    println!("{}", associated_token_account.to_string());
}
