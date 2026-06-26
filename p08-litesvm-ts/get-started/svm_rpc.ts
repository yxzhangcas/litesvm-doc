import { systemProgram } from '@solana-program/system';
import { address, createClient, generateKeyPairSigner, lamports } from '@solana/kit';
import { litesvm } from '@solana/kit-plugin-litesvm';
import { signer } from '@solana/kit-plugin-signer';

async function main() {
  const mySigner = await generateKeyPairSigner();
  const client = createClient()
    .use(signer(mySigner))
    .use(litesvm())
    .use(systemProgram());
  client.svm.airdrop(client.payer.address, lamports(10_000_000_000n));

  const myAddress = mySigner.address;
  // Set account directly via SVM
  client.svm.setAccount({
    address: myAddress,
    data: new Uint8Array([0xde, 0xad, 0xbe, 0xef]),
    executable: false,
    lamports: lamports(1_000_000n),
    programAddress: address('11111111111111111111111111111111'),
    space: 4n,
  });
  // Read via RPC (returns base64-encoded data)
  const { value } = await client.rpc.getAccountInfo(myAddress).send();
  console.log('Data:', value?.data); // ['3q2+7w==', 'base64']
}

main();