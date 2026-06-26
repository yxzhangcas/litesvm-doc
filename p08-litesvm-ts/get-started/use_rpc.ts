import { address, createClient, generateKeyPairSigner, lamports } from '@solana/kit';
import { litesvm } from '@solana/kit-plugin-litesvm';
import { signer } from '@solana/kit-plugin-signer';

async function main() {
  // Payer first, then the LiteSVM transport.
  const mySigner = await generateKeyPairSigner();
  const client = createClient()
    .use(signer(mySigner))
    .use(litesvm());

  // Fund an account
  client.svm.airdrop(mySigner.address, lamports(1_000_000_000n));

  // Set account state directly
  client.svm.setAccount({
    address: mySigner.address,
    data: new Uint8Array([1, 2, 3]),
    executable: false,
    lamports: lamports(1_000_000n),
    programAddress: address('11111111111111111111111111111111'),
    space: 3n,
  });

  // Get account info (returns base64-encoded data)
  const { value: account } = await client.rpc.getAccountInfo(mySigner.address).send();
  // Get multiple accounts in one call
  const { value: accounts } = await client.rpc
    .getMultipleAccounts([mySigner.address, mySigner.address, mySigner.address])
    .send();
  // Get latest blockhash
  const { value: blockhash } = await client.rpc.getLatestBlockhash().send();

  console.log({ account, accounts, blockhash })
}

main();