import { tokenProgram } from '@solana-program/token';
import { createClient, generateKeyPairSigner, lamports } from '@solana/kit';
import { litesvm } from '@solana/kit-plugin-litesvm';
import { signer } from '@solana/kit-plugin-signer';

async function main() {
  const mySigner = await generateKeyPairSigner();
  const client = createClient()
    .use(signer(mySigner))
    .use(litesvm())
    .use(tokenProgram());

  client.svm.airdrop(client.payer.address, lamports(10_000_000_000n));

  const mintAuthority = await generateKeyPairSigner();
  const newMint = await generateKeyPairSigner();

  await client.token.instructions
    .createMint({
      newMint,
      decimals: 9,
      mintAuthority: mintAuthority.address,
    })
    .sendTransaction();
  await client.token.instructions
    .mintToATA({
      mint: newMint.address,
      owner: client.payer.address,
      mintAuthority,
      amount: 1_000_000_000_000n, // 1000 tokens (9 decimals)
      decimals: 9,
    })
    .sendTransaction();
  // Verify
  const mintAccount = await client.token.accounts.mint.fetch(newMint.address);
  console.log(mintAccount.data.supply);  // 1000000000000n
  console.log(mintAccount.data.decimals); // 9
}

main();