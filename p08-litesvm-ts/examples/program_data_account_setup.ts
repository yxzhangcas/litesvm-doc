import { address, createClient, generateKeyPairSigner, lamports } from '@solana/kit';
import { litesvm } from '@solana/kit-plugin-litesvm';
import { signer } from '@solana/kit-plugin-signer';

async function main() {
  const mySigner = await generateKeyPairSigner();
  const client = createClient().use(signer(mySigner)).use(litesvm());

  // Your program ID
  const programId = (await generateKeyPairSigner()).address;
  const dataAccountAddress = (await generateKeyPairSigner()).address;

  // Serialize your account data structure
  // This example uses a simple layout: [u8 discriminator, u64 counter, pubkey owner]
  const data = new Uint8Array(1 + 8 + 32);
  const view = new DataView(data.buffer);

  // Set discriminator
  data[0] = 0x01;

  // Set counter (u64 little-endian)
  view.setBigUint64(1, 100n, true);

  // Set owner pubkey (32 bytes)
  // In practice, you'd decode a real address here

  // Calculate rent-exempt minimum
  const minBalance = client.svm.minimumBalanceForRentExemption(BigInt(data.length));

  // Set the account
  client.svm.setAccount({
    address: dataAccountAddress,
    data,
    executable: false,
    lamports: lamports(minBalance),
    programAddress: programId,
    space: BigInt(data.length),
  });

  console.log('Program data account set up:', dataAccountAddress);
}

main();