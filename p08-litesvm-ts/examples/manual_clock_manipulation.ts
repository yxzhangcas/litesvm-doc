import { createClient, generateKeyPairSigner } from '@solana/kit';
import { litesvm } from '@solana/kit-plugin-litesvm';
import { signer } from '@solana/kit-plugin-signer';

async function main() {
  const mySigner = await generateKeyPairSigner();
  const client = createClient().use(signer(mySigner)).use(litesvm());
  client.svm.withSysvars();

  // Get clock, mutate, write back
  const clock = client.svm.getClock();
  clock.slot = 50000n;
  clock.epoch = 5n;
  clock.unixTimestamp = 1700000000n;
  client.svm.setClock(clock);

  const after = client.svm.getClock();
  console.log('Slot:', after.slot);           // 50000n
  console.log('Epoch:', after.epoch);         // 5n
  console.log('Timestamp:', after.unixTimestamp); // 1700000000n
}

main();