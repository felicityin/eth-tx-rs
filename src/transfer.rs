use std::convert::TryFrom;
use std::str::FromStr;

use anyhow::Result;
use ethers::prelude::*;
use ethers::signers::{LocalWallet, Signer};
use ethers::types::{Address, TransactionRequest};
use ethers::types::transaction::eip2718::TypedTransaction::Legacy;

pub async fn transfer() -> Result<()> {
    // Connect to the axon network
    let provider = Provider::<Http>::try_from("http://localhost:8000")?;

    let from: Address = "0x8ab0CF264DF99D83525e9E11c7e4db01558AE1b1".parse().unwrap();
    let to: Address = "0xb91dE3190cD9b0136A05215ea5422cbB1d768926".parse().unwrap();

    let nonce = provider.get_transaction_count(from, None).await?;
    println!("nonce: {:?}", nonce);

    // Make transaction requests from accounts
    let transaction_request = TransactionRequest::new()
        .chain_id(2022)
        .to(to)
        .value(1000)
        .from(from)
        .gas_price(1)
        .gas(21000)
        .nonce(nonce);

    // Create a wallet with private key
    let wallet = LocalWallet::from_str(
        "37aa0f893d05914a4def0460c0a984d3611546cfb26924d7a7ca6e0db9950a2d",
    ).unwrap();

    // Sign the transaction
    let tx = Legacy(transaction_request);
    let signature: Signature = wallet.sign_transaction(&tx).await?;
    println!("signature: {:?}", signature);

    // Get initial balance
    let balance_before = provider.get_balance(from, None).await?;

    // Send the transaction and wait for receipt
    let receipt = provider
        .send_raw_transaction(tx.rlp_signed(&signature))
        .await?
        .await?
        .unwrap();

    println!("Executed Transaction: {:#?}", receipt);

    // Get final balance
    let balance_after = provider.get_balance(from, None).await?;

    println!("Balance before {}", balance_before);
    println!("Balance after {}", balance_after);

    Ok(())
}
