use litesvm::LiteSVM;
use solana_sdk::clock::Clock;

#[test]
fn test_time_locked_feature() {
    let mut svm = LiteSVM::new();

    // Get current time
    let clock: Clock = svm.get_sysvar();
    println!("Starting slot: {}", clock.slot);
    println!("Starting timestamp: {}", clock.unix_timestamp);

    // Test something at current time
    // ... your test logic ...

    // Jump forward 100 slots
    svm.warp_to_slot(clock.slot + 100);

    // Verify time changed
    let new_clock: Clock = svm.get_sysvar();
    assert_eq!(new_clock.slot, clock.slot + 100);

    // Test time-locked feature is now available
    // ... your test logic ...

    println!("Time travel successful!");
}