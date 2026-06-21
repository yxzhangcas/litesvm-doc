use litesvm::LiteSVM;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer, read_keypair_file},
    system_program,
    transaction::Transaction,
};
use borsh::BorshDeserialize;
use sha2::{Sha256, Digest};

// The CRUD program's DataStore account structure
#[derive(Debug, BorshDeserialize)]
struct DataStore {
    pub owner: Pubkey,
    pub title: String,
    pub message: String,
}

// Helper function to calculate instruction discriminator
// This is how Anchor generates discriminators for instructions
fn get_discriminator(instruction_name: &str) -> [u8; 8] {
    let mut hasher = Sha256::new();
    hasher.update(format!("global:{}", instruction_name));
    let result = hasher.finalize();
    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&result[..8]);
    discriminator
}

// Helper to derive the DataStore PDA
fn get_data_store_pda(title: &str, owner: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[title.as_bytes(), owner.as_ref()],
        program_id,
    )
}

// Helper to build the create instruction
fn build_create_instruction(
    program_id: &Pubkey,
    owner: &Pubkey,
    title: String,
    message: String,
) -> Instruction {
    let (data_store, _bump) = get_data_store_pda(&title, owner, program_id);

    // Build instruction data: discriminator + title length + title + message length + message
    let discriminator = get_discriminator("create");
    let mut instruction_data = Vec::new();
    instruction_data.extend_from_slice(&discriminator);

    // Serialize title (4 bytes length + string bytes)
    let title_bytes = title.as_bytes();
    instruction_data.extend_from_slice(&(title_bytes.len() as u32).to_le_bytes());
    instruction_data.extend_from_slice(title_bytes);

    // Serialize message (4 bytes length + string bytes)
    let message_bytes = message.as_bytes();
    instruction_data.extend_from_slice(&(message_bytes.len() as u32).to_le_bytes());
    instruction_data.extend_from_slice(message_bytes);

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(data_store, false),
            AccountMeta::new(*owner, true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: instruction_data,
    }
}

// Helper to build the update instruction
fn build_update_instruction(
    program_id: &Pubkey,
    owner: &Pubkey,
    title: String,
    message: String,
) -> Instruction {
    let (data_store, _bump) = get_data_store_pda(&title, owner, program_id);

    let discriminator = get_discriminator("update");
    let mut instruction_data = Vec::new();
    instruction_data.extend_from_slice(&discriminator);

    // Serialize title
    let title_bytes = title.as_bytes();
    instruction_data.extend_from_slice(&(title_bytes.len() as u32).to_le_bytes());
    instruction_data.extend_from_slice(title_bytes);

    // Serialize message
    let message_bytes = message.as_bytes();
    instruction_data.extend_from_slice(&(message_bytes.len() as u32).to_le_bytes());
    instruction_data.extend_from_slice(message_bytes);

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(data_store, false),
            AccountMeta::new(*owner, true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: instruction_data,
    }
}

// Helper to build the delete instruction
fn build_delete_instruction(
    program_id: &Pubkey,
    owner: &Pubkey,
    title: String,
) -> Instruction {
    let (data_store, _bump) = get_data_store_pda(&title, owner, program_id);

    let discriminator = get_discriminator("delete");
    let mut instruction_data = Vec::new();
    instruction_data.extend_from_slice(&discriminator);

    // Serialize title
    let title_bytes = title.as_bytes();
    instruction_data.extend_from_slice(&(title_bytes.len() as u32).to_le_bytes());
    instruction_data.extend_from_slice(title_bytes);

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(data_store, false),
            AccountMeta::new(*owner, true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: instruction_data,
    }
}

#[test]
fn test_create() {
    // Initialize LiteSVM environment
    let mut svm = LiteSVM::new();

    // Load the program from the built binary
    let program_keypair = read_keypair_file("target/deploy/crud-keypair.json")
        .expect("Failed to read program keypair");
    let program_id = program_keypair.pubkey();
    let program_bytes = include_bytes!("../target/deploy/crud.so");
    svm.add_program(program_id, program_bytes);

    // Create and fund a test user
    let owner = Keypair::new();
    svm.airdrop(&owner.pubkey(), 10_000_000_000).unwrap(); // 10 SOL

    // Test data
    let title = "My First Post".to_string();
    let message = "Hello, Solana!".to_string();

    // Derive the DataStore PDA
    let (data_store_pda, _bump) = get_data_store_pda(&title, &owner.pubkey(), &program_id);

    // Build and send the create transaction
    let create_ix = build_create_instruction(&program_id, &owner.pubkey(), title.clone(), message.clone());
    let create_tx = Transaction::new_signed_with_payer(
        &[create_ix],
        Some(&owner.pubkey()),
        &[&owner],
        svm.latest_blockhash(),
    );

    let tx_result = svm.send_transaction(create_tx);
    assert!(tx_result.is_ok(), "Create transaction failed: {:?}", tx_result.err());

    // Verify the account was created with correct data
    let account = svm.get_account(&data_store_pda)
        .expect("DataStore account should exist");

    // Account data starts with 8-byte discriminator
    let data_store = DataStore::deserialize(&mut &account.data[8..])
        .expect("Failed to deserialize DataStore");

    assert_eq!(data_store.owner, owner.pubkey(), "Owner mismatch");
    assert_eq!(data_store.title, title, "Title mismatch");
    assert_eq!(data_store.message, message, "Message mismatch");

    println!(" Create test passed!");
}

#[test]
fn test_update() {
    // Initialize LiteSVM environment
    let mut svm = LiteSVM::new();

    // Load the program
    let program_keypair = read_keypair_file("target/deploy/crud-keypair.json")
        .expect("Failed to read program keypair");
    let program_id = program_keypair.pubkey();
    let program_bytes = include_bytes!("../target/deploy/crud.so");
    svm.add_program(program_id, program_bytes);

    // Create and fund a test user
    let owner = Keypair::new();
    svm.airdrop(&owner.pubkey(), 10_000_000_000).unwrap();

    // Test data
    let title = "My Post".to_string();
    let original_message = "Original message".to_string();
    let updated_message = "Updated message".to_string();

    let (data_store_pda, _) = get_data_store_pda(&title, &owner.pubkey(), &program_id);

    // First, create the DataStore
    let create_ix = build_create_instruction(&program_id, &owner.pubkey(), title.clone(), original_message.clone());
    let create_tx = Transaction::new_signed_with_payer(
        &[create_ix],
        Some(&owner.pubkey()),
        &[&owner],
        svm.latest_blockhash(),
    );
    svm.send_transaction(create_tx).unwrap();

    // Now update the message
    let update_ix = build_update_instruction(&program_id, &owner.pubkey(), title.clone(), updated_message.clone());
    let update_tx = Transaction::new_signed_with_payer(
        &[update_ix],
        Some(&owner.pubkey()),
        &[&owner],
        svm.latest_blockhash(),
    );

    let tx_result = svm.send_transaction(update_tx);
    assert!(tx_result.is_ok(), "Update transaction failed: {:?}", tx_result.err());

    // Verify the account was updated
    let account = svm.get_account(&data_store_pda)
        .expect("DataStore account should exist");

    let data_store = DataStore::deserialize(&mut &account.data[8..])
        .expect("Failed to deserialize DataStore");

    assert_eq!(data_store.message, updated_message, "Message was not updated");
    assert_eq!(data_store.title, title, "Title should remain unchanged");

    println!(" Update test passed!");
}

#[test]
fn test_delete() {
    // Initialize LiteSVM environment
    let mut svm = LiteSVM::new();

    // Load the program
    let program_keypair = read_keypair_file("target/deploy/crud-keypair.json")
        .expect("Failed to read program keypair");
    let program_id = program_keypair.pubkey();
    let program_bytes = include_bytes!("../target/deploy/crud.so");
    svm.add_program(program_id, program_bytes);

    // Create and fund a test user
    let owner = Keypair::new();
    svm.airdrop(&owner.pubkey(), 10_000_000_000).unwrap();

    // Test data
    let title = "Post to Delete".to_string();
    let message = "This will be deleted".to_string();

    let (data_store_pda, _) = get_data_store_pda(&title, &owner.pubkey(), &program_id);

    // First, create the DataStore
    let create_ix = build_create_instruction(&program_id, &owner.pubkey(), title.clone(), message);
    let create_tx = Transaction::new_signed_with_payer(
        &[create_ix],
        Some(&owner.pubkey()),
        &[&owner],
        svm.latest_blockhash(),
    );
    svm.send_transaction(create_tx).unwrap();

    // Verify account exists before deletion
    assert!(svm.get_account(&data_store_pda).is_some(), "Account should exist before deletion");

    // Get owner's balance before deletion (should receive rent back)
    let balance_before = svm.get_balance(&owner.pubkey()).unwrap();

    // Now delete the DataStore
    let delete_ix = build_delete_instruction(&program_id, &owner.pubkey(), title);
    let delete_tx = Transaction::new_signed_with_payer(
        &[delete_ix],
        Some(&owner.pubkey()),
        &[&owner],
        svm.latest_blockhash(),
    );

    let tx_result = svm.send_transaction(delete_tx);
    assert!(tx_result.is_ok(), "Delete transaction failed: {:?}", tx_result.err());

    // Verify the account was deleted (closed) - closed accounts have 0 lamports
    let account = svm.get_account(&data_store_pda);
    match account {
        Some(acc) => assert_eq!(acc.lamports, 0, "Account should have 0 lamports (closed)"),
        None => {} // Account removed completely is also valid
    }

    // Verify owner received rent back (balance should increase)
    let balance_after = svm.get_balance(&owner.pubkey()).unwrap();
    assert!(balance_after > balance_before, "Owner should receive rent back after closing account");

    println!(" Delete test passed!");
}

#[test]
fn test_crud_full_lifecycle() {
    // This test demonstrates the full CRUD lifecycle:
    // Create -> Read -> Update -> Read -> Delete

    let mut svm = LiteSVM::new();

    // Load the program
    let program_keypair = read_keypair_file("target/deploy/crud-keypair.json")
        .expect("Failed to read program keypair");
    let program_id = program_keypair.pubkey();
    let program_bytes = include_bytes!("../target/deploy/crud.so");
    svm.add_program(program_id, program_bytes);

    // Create and fund a test user
    let owner = Keypair::new();
    svm.airdrop(&owner.pubkey(), 10_000_000_000).unwrap();

    let title = "Full Lifecycle Test".to_string();
    let (data_store_pda, _) = get_data_store_pda(&title, &owner.pubkey(), &program_id);

    // Step 1: CREATE
    println!("Step 1: Creating DataStore...");
    let create_message = "Initial message".to_string();
    let create_ix = build_create_instruction(&program_id, &owner.pubkey(), title.clone(), create_message.clone());
    let create_tx = Transaction::new_signed_with_payer(
        &[create_ix],
        Some(&owner.pubkey()),
        &[&owner],
        svm.latest_blockhash(),
    );
    svm.send_transaction(create_tx).expect("Create failed");

    // Step 2: READ (verify creation)
    println!("Step 2: Reading DataStore after creation...");
    let account = svm.get_account(&data_store_pda).expect("Account should exist");
    let data_store = DataStore::deserialize(&mut &account.data[8..]).unwrap();
    assert_eq!(data_store.title, title);
    assert_eq!(data_store.message, create_message);
    assert_eq!(data_store.owner, owner.pubkey());

    // Step 3: UPDATE
    println!("Step 3: Updating DataStore message...");
    let updated_message = "Updated message!".to_string();
    let update_ix = build_update_instruction(&program_id, &owner.pubkey(), title.clone(), updated_message.clone());
    let update_tx = Transaction::new_signed_with_payer(
        &[update_ix],
        Some(&owner.pubkey()),
        &[&owner],
        svm.latest_blockhash(),
    );
    svm.send_transaction(update_tx).expect("Update failed");

    // Step 4: READ (verify update)
    println!("Step 4: Reading DataStore after update...");
    let account = svm.get_account(&data_store_pda).expect("Account should exist");
    let data_store = DataStore::deserialize(&mut &account.data[8..]).unwrap();
    assert_eq!(data_store.message, updated_message);

    // Step 5: DELETE
    println!("Step 5: Deleting DataStore...");
    let delete_ix = build_delete_instruction(&program_id, &owner.pubkey(), title.clone());
    let delete_tx = Transaction::new_signed_with_payer(
        &[delete_ix],
        Some(&owner.pubkey()),
        &[&owner],
        svm.latest_blockhash(),
    );
    svm.send_transaction(delete_tx).expect("Delete failed");

    // Step 6: Verify deletion - closed accounts have 0 lamports
    println!("Step 6: Verifying DataStore was deleted...");
    let account = svm.get_account(&data_store_pda);
    match account {
        Some(acc) => assert_eq!(acc.lamports, 0, "Account should have 0 lamports (closed)"),
        None => {} // Account removed completely is also valid
    }

    println!(" Full lifecycle test passed!");
}

#[test]
fn test_multiple_data_stores() {
    // Test that multiple DataStores can be created with different titles
    let mut svm = LiteSVM::new();

    let program_keypair = read_keypair_file("target/deploy/crud-keypair.json")
        .expect("Failed to read program keypair");
    let program_id = program_keypair.pubkey();
    let program_bytes = include_bytes!("../target/deploy/crud.so");
    svm.add_program(program_id, program_bytes);

    let owner = Keypair::new();
    svm.airdrop(&owner.pubkey(), 10_000_000_000).unwrap();

    // Create multiple DataStores with different titles
    let posts = vec![
        ("First Post", "First message"),
        ("Second Post", "Second message"),
        ("Third Post", "Third message"),
    ];

    for (title, message) in &posts {
        let create_ix = build_create_instruction(
            &program_id,
            &owner.pubkey(),
            title.to_string(),
            message.to_string(),
        );
        let create_tx = Transaction::new_signed_with_payer(
            &[create_ix],
            Some(&owner.pubkey()),
            &[&owner],
            svm.latest_blockhash(),
        );
        svm.send_transaction(create_tx)
            .expect(&format!("Failed to create post: {}", title));
    }

    // Verify all posts exist and have correct data
    for (title, message) in &posts {
        let (data_store_pda, _) = get_data_store_pda(title, &owner.pubkey(), &program_id);
        let account = svm.get_account(&data_store_pda)
            .expect(&format!("Account for '{}' should exist", title));

        let data_store = DataStore::deserialize(&mut &account.data[8..]).unwrap();
        assert_eq!(data_store.title, *title);
        assert_eq!(data_store.message, *message);
    }

    println!(" Multiple data stores test passed!");
}
