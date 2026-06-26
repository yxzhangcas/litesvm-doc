import { createClient, address, generateKeyPairSigner, lamports } from '@solana/kit';
import { litesvm } from '@solana/kit-plugin-litesvm';
import { signer } from '@solana/kit-plugin-signer';

async function main() {
const mySigner = await generateKeyPairSigner();
const client = createClient().use(signer(mySigner)).use(litesvm());
const programId = address('11111111111111111111111111111111');

// Set up multiple accounts with different data
const account1 = (await generateKeyPairSigner()).address;
const account2 = (await generateKeyPairSigner()).address;
const account3 = (await generateKeyPairSigner()).address;
const account4 = (await generateKeyPairSigner()).address;
const account5 = (await generateKeyPairSigner()).address;
const accounts = [account1, account2, account3, account4, account5];

for (const [index, accountAddress] of accounts.entries()) {
    const customData = new Uint8Array([index, index + 1, index + 2]);

    client.svm.setAccount({
        address: accountAddress,
        data: customData,
        executable: false,
        lamports: lamports(1_000_000n),
        programAddress: programId,
        space: BigInt(customData.length),
    });

    console.log(`Account ${index}: ${accountAddress}`);
}

// Verify accounts via RPC
console.log('\nVerifying via RPC:');
const { value: rpcAccounts } = await client.rpc
    .getMultipleAccounts(accounts)
    .send();

rpcAccounts.forEach((acc, i) => {
    if (acc) {
        console.log(`  Account ${i}: ${acc.lamports} lamports, data: ${acc.data[0]}`);
    }
});
}

main();