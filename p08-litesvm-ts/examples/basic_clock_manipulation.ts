import { createClient, generateKeyPairSigner } from '@solana/kit';
import { litesvm } from '@solana/kit-plugin-litesvm';
import { signer } from '@solana/kit-plugin-signer';

async function main() {
  const mySigner = await generateKeyPairSigner();
  const client = createClient().use(signer(mySigner)).use(litesvm());

  // Enable sysvars for clock access
  client.svm.withSysvars();

  // Check initial clock
  const initialClock = client.svm.getClock();
  console.log('Initial state:');
  console.log('  Slot:', initialClock.slot);
  console.log('  Epoch:', initialClock.epoch);
  console.log('  Unix timestamp:', initialClock.unixTimestamp);

  // Warp to slot 1000
  client.svm.warpToSlot(1000n);

  const afterWarp = client.svm.getClock();
  console.log('\nAfter warpToSlot(1000):');
  console.log('  Slot:', afterWarp.slot);
  console.log('  Epoch:', afterWarp.epoch);

  // Warp further
  client.svm.warpToSlot(100000n);

  const afterWarp2 = client.svm.getClock();
  console.log('\nAfter warpToSlot(100000):');
  console.log('  Slot:', afterWarp2.slot);
  console.log('  Epoch:', afterWarp2.epoch);
}

main();