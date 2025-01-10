mod programs;

#[cfg(test)]

const RPC_URL: &str = "https://api.devnet.solana.com";
mod tests {
    use bs58;
    use solana_client::{nonblocking::rpc_client, rpc_client::RpcClient};
    use solana_program::{pubkey::Pubkey, system_instruction::transfer};
    use solana_sdk::{
        message::Message,
        signature::{self, read_keypair_file, Keypair, Signer},
        signer::keypair,
        system_program,
        sysvar::recent_blockhashes,
        transaction::{get_nonce_pubkey_from_instruction, Transaction},
    };
    use std::str::FromStr;
    use std::{
        io::{self, BufRead},
        vec,
    };

    use crate::programs::Turbin3_prereq::{CompleteArgs, TurbinePrereqProgram, UpdateArgs};

    use crate::RPC_URL;

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!(
            "You've generated a new Solana wallet: {}",
            kp.pubkey().to_string()
        );
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");

        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a wallet file byte array:");
        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches("[")
            .trim_end_matches("]")
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        println!("Your private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }

    #[test]
    fn airdrop() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);
        println!("{}", &keypair.pubkey().to_string());
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Checkout your TX here:");
                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    s.to_string()
                );
            }
            Err(e) => println!("Oops, something went wrong: {}", e.to_string()),
        };
    }

    #[test]
    fn transfer_sol() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet File");
        let to_pubkey = Pubkey::from_str("FNwKzzuFLFiyfX4rwegsytSDsM6KHmee3mL94hejukSr").unwrap();

        // Create a Solana Devnet connection
        let rpc_client = RpcClient::new(RPC_URL);

        // Get Balance of Dev Wallet
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get Balance");

        // Get recent Blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent Blockhash");

        // Create a test transaction to calulcate fees
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );

        // Calculate exact fee rate to transfer entire SOL amount minus fees
        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee Calculator");

        // Deduct fee from lamports amount and create a TX with correct balance
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        // Send the Transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        // Print out our Transaction
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn enroll() {
        //Create Devnet Connection
        let rpc_client = RpcClient::new(RPC_URL);

        // Let's define our accounts
        let signer =
            read_keypair_file("turbin3dev-wallet.json").expect("Couldn't find wallet file");

        // Create a PDA
        let prereq = TurbinePrereqProgram::derive_program_address(&[
            b"prereq",
            signer.pubkey().to_bytes().as_ref(),
        ]);

        // Define out instruction data
        let args = CompleteArgs {
            github: b"Gator-RR".to_vec(),
        };

        // Get Recent Blockhash
        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // Invoke the "complete" function
        let transaction = TurbinePrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash,
        );

        // Send Transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        // Printout transaction
        println!("Success! Check out your TX here:https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
    }
}
