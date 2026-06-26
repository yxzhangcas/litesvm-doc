import { findAssociatedTokenPda, TOKEN_PROGRAM_ADDRESS, tokenProgram } from '@solana-program/token';
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

  const alice = await generateKeyPairSigner();
  const bob = await generateKeyPairSigner();
  // Mint to Alice
  await client.token.instructions
    .mintToATA({
      mint: newMint.address,
      owner: alice.address,
      mintAuthority,
      amount: 1_000_000_000_000n,
      decimals: 9,
    })
    .sendTransaction();
  // Transfer 400 tokens from Alice to Bob
  await client.token.instructions
    .transferToATA({
      mint: newMint.address,
      authority: alice,
      recipient: bob.address,
      amount: 400_000_000_000n,
      decimals: 9,
    })
    .sendTransaction();

  const [aliceAta] = await findAssociatedTokenPda({
    owner: alice.address,
    mint: newMint.address,
    tokenProgram: TOKEN_PROGRAM_ADDRESS,
  });
  const aliceAccount = await client.token.accounts.token.fetch(aliceAta);
  console.log(aliceAccount.data.amount); // 600_000_000_000n
  console.log(aliceAccount.data.mint);   // newMint.address
  console.log(aliceAccount.data.owner);  // alice.address
}

main();