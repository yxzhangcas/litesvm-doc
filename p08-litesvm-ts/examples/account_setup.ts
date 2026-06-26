import { address, createClient, generateKeyPairSigner, lamports } from '@solana/kit';
import { litesvm } from '@solana/kit-plugin-litesvm';
import { signer } from '@solana/kit-plugin-signer';

async function main() {
  const mySigner = await generateKeyPairSigner();
  const client = createClient().use(signer(mySigner)).use(litesvm());

  // Calculate minimum balance for rent exemption
  const dataSize = 100n;
  const minBalance = client.svm.minimumBalanceForRentExemption(dataSize);
  console.log(`Minimum balance for ${dataSize} bytes:`, minBalance, 'lamports');

  // Set account state directly (no transaction needed)
  const programId = address('11111111111111111111111111111111');
  const accountData = new Uint8Array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

  const testAddress = (await generateKeyPairSigner()).address;

  client.svm.setAccount({
    address: testAddress,
    data: accountData,
    executable: false,
    lamports: lamports(minBalance),
    programAddress: programId,
    space: BigInt(accountData.length),
  });

  console.log('Account state set successfully');

  // Retrieve the account data
  const retrieved = client.svm.getAccount(testAddress);

  if (retrieved.exists) {
    console.log('Retrieved account:');
    console.log('  Address:', retrieved.address);
    console.log('  Lamports:', retrieved.lamports);
    console.log('  Owner:', retrieved.programAddress);
    console.log('  Executable:', retrieved.executable);
    console.log('  Data:', Array.from(retrieved.data));
  }

  // Get balance separately
  const balance = client.svm.getBalance(testAddress);
  console.log('Balance:', balance, 'lamports');
}

main();