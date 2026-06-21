// use litesvm::LiteSVM;
// use solana_sdk::{
//     instruction::{AccountMeta, Instruction},
//     pubkey::Pubkey,
//     signature::{Keypair, Signer},
// };
// use solana_system_interface::program;
// use solana_transaction::Transaction;

// #[test]
// fn test_cross_program_invocation() {
//     let mut svm = LiteSVM::new();

//     // Deploy both programs
//     let caller_program = Pubkey::new_unique();
//     let callee_program = Pubkey::new_unique();

//     svm.add_program(
//         caller_program,
//         include_bytes!("../target/deploy/caller.so")
//     ).unwrap();

//     svm.add_program(
//         callee_program,
//         include_bytes!("../target/deploy/callee.so")
//     ).unwrap();

//     // Setup accounts
//     let payer = Keypair::new();
//     svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();

//     // Create instruction that will trigger CPI
//     let instruction = Instruction {
//         program_id: caller_program,
//         accounts: vec![
//             AccountMeta::new(payer.pubkey(), true),
//             AccountMeta::new_readonly(callee_program, false),
//             AccountMeta::new_readonly(program::id(), false),
//         ],
//         data: vec![1], // Instruction to trigger CPI
//     };

//     let tx = Transaction::new_signed_with_payer(
//         &[instruction],
//         Some(&payer.pubkey()),
//         &[&payer],
//         svm.latest_blockhash(),
//     );

//     let result = svm.send_transaction(tx).unwrap();

//     // Verify both programs were invoked
//     assert!(result.logs.iter().any(|log|
//         log.contains(&format!("Program {} invoke", caller_program))
//     ));
//     assert!(result.logs.iter().any(|log|
//         log.contains(&format!("Program {} invoke", callee_program))
//     ));

//     println!("CPI successful!");
//     println!("Logs showing both programs:");
//     for log in &result.logs {
//         if log.contains("invoke") {
//             println!("   {}", log);
//         }
//     }
// }