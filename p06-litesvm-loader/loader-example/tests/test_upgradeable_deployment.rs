use litesvm::LiteSVM;
use litesvm_loader::{deploy_upgradeable_program, set_upgrade_authority};
use solana_keypair::Keypair;
use solana_signer::Signer;

#[test]
fn test_upgradeable_deployment() {
    let mut svm = LiteSVM::new();

    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();

    // Use the keypair that should own the program ID.
    // For Anchor programs, this is usually target/deploy/<program>-keypair.json.
    let program = Keypair::new();
    let program_bytes = include_bytes!("../target/deploy/loader_example.so");

    deploy_upgradeable_program(&mut svm, &payer, &program, program_bytes).unwrap();

    let program_account = svm.get_account(&program.pubkey()).unwrap();
    assert!(program_account.executable);

    let new_authority = Keypair::new();
    set_upgrade_authority(
        &mut svm,
        &payer,
        &program.pubkey(),
        &payer,
        Some(&new_authority.pubkey()),
    )
    .unwrap();
}