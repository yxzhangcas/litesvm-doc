use litesvm::LiteSVM;
use solana_sdk::signature::{read_keypair_file, Signer};

#[test]
fn test_deploy_from_bytes() {
    let mut svm = LiteSVM::new();

    // Read the program's keypair to get correct ID
    // 此处是相对Cargo.toml的路径
    let program_keypair = read_keypair_file("../../target/deploy/sample-keypair.json")
        .expect("Program keypair file not found");
    let program_id = program_keypair.pubkey();

    // Include bytes at compile time
    // 此处是相对rs文件的路径
    let program_bytes = include_bytes!("../../../target/deploy/sample.so");
    // Deploy from bytes
    svm.add_program(program_id, program_bytes)
        .expect("Failed to deploy program");

    // Verify deployment
    assert!(
        svm.get_account(&program_id).is_some(),
        "Program account not created"
    );
    assert!(
        svm.get_account(&program_id).unwrap().executable,
        "Program not executable"
    );
}
