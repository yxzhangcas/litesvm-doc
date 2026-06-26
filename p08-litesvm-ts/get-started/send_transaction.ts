import { systemProgram } from '@solana-program/system';
import { createClient, generateKeyPairSigner, lamports } from '@solana/kit';
import { litesvm } from '@solana/kit-plugin-litesvm';
import { signer } from '@solana/kit-plugin-signer';

async function main() {
  const mySigner = await generateKeyPairSigner();
  const client = createClient()
    .use(signer(mySigner))
    .use(litesvm())
    .use(systemProgram());
  client.svm.airdrop(client.payer.address, lamports(10_000_000_000n));
  const recipient = await generateKeyPairSigner();
  // Build and send in one call
  await client.system.instructions.transferSol({
    source: client.payer,
    destination: recipient.address,
    amount: lamports(1_000_000_000n),
  }).sendTransaction();

  const payerBalance = await client.svm.getBalance(mySigner.address);
  const recipientBalance = await client.svm.getBalance(recipient.address);
  console.log({ payerBalance, recipientBalance });
}

main();