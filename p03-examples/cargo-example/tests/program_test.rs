// use litesvm::LiteSVM;
// use solana_sdk::{
//     instruction::{AccountMeta, Instruction},
//     pubkey::Pubkey,
//     signature::{Keypair, Signer},
// };
// use solana_transaction::Transaction;

// #[test]
// fn test_program_deployment() {
//     let mut svm = LiteSVM::new();

//     // Deploy the program
//     let program_id = Pubkey::new_unique();
//     let program_bytes = include_bytes!("../target/deploy/hello_world.so");  // 无此so文件
//     svm.add_program(program_id, program_bytes).unwrap();

//     // Create payer account
//     let payer = Keypair::new();
//     svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();

//     // Create instruction to call program
//     let instruction = Instruction {
//         program_id,
//         accounts: vec![AccountMeta::new(payer.pubkey(), true)],
//         data: vec![], // Program-specific data
//     };

//     // Send transaction
//     let tx = Transaction::new_signed_with_payer(
//         &[instruction],
//         Some(&payer.pubkey()),
//         &[&payer],
//         svm.latest_blockhash(),
//     );

//     let result = svm.send_transaction(tx).unwrap();

//     // Verify program was called
//     assert!(
//         result
//             .logs
//             .iter()
//             .any(|log| log.contains(&format!("Program {} invoke", program_id)))
//     );

//     println!("Program called successfully!");
//     println!("Logs:\n{}", result.pretty_logs());
// }
