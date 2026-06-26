import { createClient, generateKeyPairSigner, lamports } from '@solana/kit';
import { litesvm } from '@solana/kit-plugin-litesvm';
import { signer } from '@solana/kit-plugin-signer';

async function main() {
  // Payer first, then the LiteSVM transport.
  const mySigner = await generateKeyPairSigner();
  const client = createClient()
    .use(signer(mySigner))
    .use(litesvm());

  // Fund the payer
  client.svm.airdrop(client.payer.address, lamports(5_000_000_000n));

  // Check the balance
  const balance = client.svm.getBalance(client.payer.address);
  console.log('Balance:', balance, 'lamports');
}

main();