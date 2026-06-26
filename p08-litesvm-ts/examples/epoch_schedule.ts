import { createClient, generateKeyPairSigner } from '@solana/kit';
import { litesvm } from '@solana/kit-plugin-litesvm';
import { signer } from '@solana/kit-plugin-signer';

async function main() {
  const mySigner = await generateKeyPairSigner();
  const client = createClient().use(signer(mySigner)).use(litesvm());
  client.svm.withSysvars();

  const epochSchedule = client.svm.getEpochSchedule();
  console.log('Slots per epoch:', epochSchedule.slotsPerEpoch);
  console.log('First normal epoch:', epochSchedule.firstNormalEpoch);
  console.log('First normal slot:', epochSchedule.firstNormalSlot);
}

main();