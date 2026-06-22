use anchor_litesvm::AnchorLiteSVM;
use anchor_litesvm::TestHelpers;
use anchor_lang::system_program;
use solana_sdk::signature::{read_keypair_file, Signer};
use anchor_lang::prelude::*;

// Generate client types from your program's IDL
anchor_lang::declare_program!(anchor_litesvm_example);


#[test]
fn test_anchor_program() {
    // One-line setup — reads program keypair for the correct ID
    let program_keypair = read_keypair_file("../../target/deploy/anchor_litesvm_example-keypair.json").unwrap();
    let mut ctx = AnchorLiteSVM::build_with_program(
        program_keypair.pubkey(),
        include_bytes!("../../../target/deploy/anchor_litesvm_example.so"),
    );

    // Create a funded account via TestHelpers on ctx.svm
    let user = ctx.svm.create_funded_account(10_000_000_000).unwrap();

    // Derive PDA
    let seed: u64 = 42;
    let pda = ctx.svm.get_pda(
        &[b"user", user.pubkey().as_ref(), &seed.to_le_bytes()],
        &program_keypair.pubkey(),
    );

    // Build instruction using generated client types
    let ix = ctx.program()
        .accounts(anchor_litesvm_example::client::accounts::Initialize {
            user: user.pubkey(),
            user_account: pda,
            system_program: system_program::ID,
        })
        .args(anchor_litesvm_example::client::args::Initialize {
            seed,
            name: "test".to_string(),
        })
        .instruction()
        .unwrap();

    // Execute and assert in one chain
    ctx.execute_instruction(ix, &[&user])
        .unwrap()
        .assert_success();

    // Fetch and deserialize the account
    let account: anchor_litesvm_example::accounts::MyAccount = ctx.get_account(&pda).unwrap();
    println!("{}", account.name);
    assert_eq!(account.name, "test");
}