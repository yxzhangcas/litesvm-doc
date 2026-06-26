import { address, createClient, generateKeyPairSigner, lamports } from '@solana/kit';
import { litesvm } from '@solana/kit-plugin-litesvm';
import { signer } from '@solana/kit-plugin-signer';

async function main() {
  const mySigner = await generateKeyPairSigner();
  const client = createClient().use(signer(mySigner)).use(litesvm());

  client.svm
    .withSigverify(false)
    .withBlockhashCheck(false)
    .withSysvars();

  // Fund payer
  client.svm.airdrop(client.payer.address, lamports(10_000_000_000n));

  // Set up a time-locked vault account
  // Structure: [u8 discriminator, u64 unlock_slot, u64 amount]
  const unlockSlot = 10000n;
  const lockedAmount = 5_000_000_000n;

  const vaultData = new Uint8Array(1 + 8 + 8);
  const view = new DataView(vaultData.buffer);
  vaultData[0] = 0x01; // discriminator
  view.setBigUint64(1, unlockSlot, true); // unlock_slot
  view.setBigUint64(9, lockedAmount, true); // amount

  const programId = (await generateKeyPairSigner()).address;
  const vaultAddress = (await generateKeyPairSigner()).address;
  const minBalance = client.svm.minimumBalanceForRentExemption(BigInt(vaultData.length));

  client.svm.setAccount({
    address: vaultAddress,
    data: vaultData,
    executable: false,
    lamports: lamports(minBalance + lockedAmount),
    programAddress: programId,
    space: BigInt(vaultData.length),
  });

  console.log('Vault created with unlock slot:', unlockSlot);
  console.log('Locked amount:', Number(lockedAmount) / 1e9, 'SOL');

  // Test 1: Try to withdraw before unlock (should fail)
  console.log('\n--- Test 1: Before unlock ---');
  const currentClock = client.svm.getClock();
  console.log('Current slot:', currentClock.slot);

  // In a real test, you would:
  // 1. Build a withdraw instruction
  // 2. Send it and expect it to fail
  console.log('Withdrawal attempt: EXPECTED TO FAIL (too early)');

  // Test 2: Warp to after unlock slot
  console.log('\n--- Test 2: After unlock ---');
  client.svm.warpToSlot(unlockSlot + 100n);

  const newClock = client.svm.getClock();
  console.log('Current slot:', newClock.slot);

  // In a real test:
  // 1. Build a withdraw instruction
  // 2. Send it and expect it to succeed
  console.log('Withdrawal attempt: EXPECTED TO SUCCEED (after unlock)');

  // Verify vault state
  const updatedVault = client.svm.getAccount(vaultAddress);
  if (updatedVault.exists) {
    console.log('\nVault balance:', updatedVault.lamports);
  }
}

main();