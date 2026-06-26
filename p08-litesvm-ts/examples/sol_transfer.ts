import { systemProgram } from '@solana-program/system';
import { createClient, generateKeyPairSigner, lamports } from '@solana/kit';
import { litesvm } from '@solana/kit-plugin-litesvm';
import { signer } from '@solana/kit-plugin-signer';

async function main() {
  // Payer first, then the LiteSVM transport, then program plugins.
  const mySigner = await generateKeyPairSigner();
  const client = createClient()
    .use(signer(mySigner))
    .use(litesvm())
    .use(systemProgram());
  const recipient = await generateKeyPairSigner();

  // Airdrop SOL to the payer
  client.svm.airdrop(client.payer.address, lamports(10_000_000_000n));

  // Check initial balance
  const senderInitial = client.svm.getBalance(client.payer.address) ?? 0n;
  console.log('Sender initial:', Number(senderInitial) / 1e9, 'SOL');

  // Build and send the transfer
  await client.system.instructions.transferSol({
    source: client.payer,
    destination: recipient.address,
    amount: lamports(1_000_000_000n), // 1 SOL
  }).sendTransaction();

  // Check final balances
  const senderFinal = client.svm.getBalance(client.payer.address) ?? 0n;
  const recipientFinal = client.svm.getBalance(recipient.address) ?? 0n;
  console.log('Sender final:', Number(senderFinal) / 1e9, 'SOL');
  console.log('Recipient final:', Number(recipientFinal) / 1e9, 'SOL');
}

main();