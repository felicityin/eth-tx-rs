use std::convert::TryFrom;
use std::str::FromStr;

use anyhow::Result;
use ethers::abi::AbiEncode;
use ethers::prelude::*;
use ethers::signers::{LocalWallet, Signer};
use ethers::types::{Address, TransactionRequest};
use ethers::types::transaction::eip2718::TypedTransaction::Legacy;

use crate::image_cell_abi;

pub async fn send_tx() -> Result<()> {
    // Connect to the axon network
    let provider = Provider::<Http>::try_from("http://localhost:8000")?;

    let from: Address = "0x8ab0CF264DF99D83525e9E11c7e4db01558AE1b1".parse().unwrap();
    let to: Address = system_contract_address(0x1);

    let nonce = provider.get_transaction_count(from, None).await?;
    println!("nonce: {:?}", nonce);

    // Data
    let data = image_cell_abi::UpdateCall {
        header:  image_cell_abi::Header {
            version:           0x0,
            compact_target:    0x1a9c7b1a,
            timestamp:         0x16e62df76ed,
            number:            0x1,
            epoch:             0x7080291000049,
            parent_hash:       [0u8; 32],
            transactions_root: [1u8; 32],
            proposals_hash:    [2u8; 32],
            uncles_hash:       [3u8; 32],
            dao:               [4u8; 32],
            nonce:             0x78b105de64fc38a200000004139b0200,
            block_hash:        [5u8; 32],
        },
        inputs:  vec![image_cell_abi::OutPoint {
            tx_hash: [7u8; 32],
            index:   0x0,
        }],
        outputs: vec![image_cell_abi::CellInfo {
            out_point: image_cell_abi::OutPoint {
                tx_hash: [7u8; 32],
                index:   0x0,
            },
            output:    image_cell_abi::CellOutput {
                capacity: 0x34e62ce00,
                lock:     image_cell_abi::Script {
                    args:      ethers::core::types::Bytes::from_str(
                        "0x927f3e74dceb87c81ba65a19da4f098b4de75a0d",
                    )
                    .unwrap(),
                    code_hash: [8u8; 32],
                    hash_type: 1,
                },
                type_:    vec![image_cell_abi::Script {
                    args:      ethers::core::types::Bytes::from_str(
                        "0x6e9b17739760ffc617017f157ed40641f7aa51b2af9ee017b35a0b35a1e2297b",
                    )
                    .unwrap(),
                    code_hash: [9u8; 32],
                    hash_type: 0,
                }],
            },
            data:      ethers::core::types::Bytes::from_str("0x40420f00000000000000000000000000")
                .unwrap(),
        }],
    }.encode();

    // Make transaction requests from accounts
    let transaction_request = TransactionRequest::new()
        .chain_id(2022)
        .to(to)
        .data(data)
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


const fn system_contract_address(addr: u8) -> H160 {
    H160([
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, addr,
    ])
}
