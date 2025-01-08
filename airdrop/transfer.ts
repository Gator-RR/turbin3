import {
  Transaction,
  SystemProgram,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  sendAndConfirmTransaction,
  PublicKey,
} from "@solana/web3.js";

import wallet from "./dev-wallet.json"


const from = Keypair.fromSecretKey(new Uint8Array(wallet));

const to = new PublicKey("FNwKzzuFLFiyfX4rwegsytSDsM6KHmee3mL94hejukSr");

const connection = new Connection("https://api.devnet.solana.com");

(async () => {
    try {
        //Get Balance
        const balance = await connection.getBalance(from.publicKey)

        const transaction = new Transaction().add(
            SystemProgram.transfer({
                fromPubkey: from.publicKey,
                toPubkey: to,
                lamports: balance,
            })
        );
        transaction.recentBlockhash = (await connection.getLatestBlockhash('confirmed')).blockhash;
        transaction.feePayer = from.publicKey;

        // Calculate exact fee rate to transfer entire SOL amount minus fees
        const fee = (await connection.getFeeForMessage(transaction.compileMessage(),'confirmed')).value || 0;
        
        // Remove our transfer instruction to replace
        transaction.instructions.pop();

        // add the instruction back with correct amount
        transaction.add(
            SystemProgram.transfer({
                fromPubkey: from.publicKey,
                toPubkey: to,
                lamports: balance - fee,
            })
        );

        // Sign and confirm
        const signature = await sendAndConfirmTransaction(
            connection,
            transaction,
            [from]
        );
        console.log(`Success! Check out your TX here: https://explorer.solana.com/tx/${signature}?cluster=devnet`);
    } catch (e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();