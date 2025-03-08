mod core;
mod utils;
use core::blockchain_manager::BlockchainManager;
use rand::distr::{Distribution, Uniform};
use std::io;
use utils::hash::bytes_to_hex_string;

fn main() {
    let mut rng = rand::rng();
    let mut blockchain_manager = match BlockchainManager::new("blockchain_db") {
        Ok(blockchain_manager) => blockchain_manager,
        Err(err) => {
            println!("Failed to initialize blockchain manager: {}", err);
            return;
        }
    };
    let mut blockchain = blockchain_manager.get_blockchain();
    loop {
        show();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse() {
            Ok(0) => {
                blockchain_manager.blockchain = blockchain;
                let _ = blockchain_manager.save();
                println!("Exiting application. Blockchain saved.");
                break;
            }
            Ok(1) => {
                println!("Generating new block with random transactions...");
                let mut transactions: Vec<String> = Vec::new();
                let die = Uniform::new_inclusive(1, 100);
                let num = die.unwrap().sample(&mut rng);
                for i in 0..num {
                    transactions.push(format!("transaction {}", i));
                }
                let _ = blockchain.add_block(transactions);
                println!("New block successfully mined and added to the chain.");
            }
            Ok(2) => {
                let blockchain_iter = blockchain.iter();
                blockchain_iter.for_each(|block| {
                    println!("[Block Details]");
                    println!("Timestamp: {}", block.header.timestamp);
                    println!(
                        "Previous Hash: {}",
                        bytes_to_hex_string(&block.header.prev_hash)
                    );
                    println!("Current Hash: {}", bytes_to_hex_string(&block.hash));
                    println!("Nonce: {}", block.header.nonce);
                    println!("Transaction Count: {}", block.transactions.len());
                    println!("Transactions:");
                    for (i, tx) in block.transactions.iter().enumerate() {
                        println!(" {}. {}", i + 1, tx);
                    }
                    println!("-----------------------------");
                });
            }
            _ => {}
        }
    }
}
fn show() {
    println!("Blockchain CLI - Main Menu");
    println!("1. Generate new block");
    println!("2. Display blockchain");
    println!("0. Exit and save");
    println!("Enter your choice: ");
}
