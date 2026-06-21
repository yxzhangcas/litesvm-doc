use litesvm::LiteSVM;
use solana_sdk::signature::{read_keypair_file, Signer};

#[test]
fn test_deploy_from_file() {
    let mut svm = LiteSVM::new();

    // Read keypair for correct program ID
    // 此处是相对Cargo.toml的路径
    let program_keypair = read_keypair_file("../../target/deploy/sample-keypair.json").unwrap();
    let program_id = program_keypair.pubkey();

    // Deploy from file
    // 此处是相对Cargo.toml的路径
    svm.add_program_from_file(program_id, "../../target/deploy/sample.so")
        .expect("Failed to deploy program from file");

    // Always verify
    assert!(svm.get_account(&program_id).unwrap().executable);
}
