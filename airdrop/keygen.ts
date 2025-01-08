import { Keypair } from "@solana/web3.js";
// import { bs58 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import bs58 from 'bs58';
import test from "node:test";
import promptSync from 'prompt-sync';

//Generate a new Keypair
let kp = Keypair.generate()
console.log(`You've generated a new Solana wallet: ${kp.publicKey.toBase58()}`)
console.log(`[${kp.secretKey}]`)

const prompt = promptSync();

const walletBytes = new Uint8Array([83,0,63,82,205,54,171,172,65,2,162,71,17,59,140,90,253,110,38,180,86,139,110,121,1,154,91,198,4,171,189,202,13,33,200,98,51,32,16,134,110,111,112,159,211,165,228,186,10,221,159,148,90,237,253,64,161,119,49,127,78,90,234,12]);
    
const base58String = bs58.encode(walletBytes);
console.log("Base58 encoded string:", base58String);
// return base58String;

// @test
// function base58ToWallet() {
//     console.log("Enter base58 string:");
//     const base58Input = prompt('> ');
//     try {
//         const wallet = bs58.decode(base58Input);
//         console.log("Decoded wallet bytes:", wallet);
//         return wallet;
//     } catch (error) {
//         console.error("Error decoding base58:", error);
//         return null;
//     }
// }

// @test
// function walletToBase58() {
//     // Example wallet bytes (you can modify this to read from stdin/file)

// }
