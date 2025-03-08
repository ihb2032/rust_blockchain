mod core;
mod utils;
use core::blockchain_manager::BlockchainManager;
use rand::distr::{Distribution, Uniform};
use std::io;
use utils::hash::bytes_to_hex_string;

fn main() {
    let mut rng = rand::rng();
    let blockchain_manager = match BlockchainManager::new("blockchain_db") {
        Ok(blockchain_manager) => blockchain_manager,
        Err(err) => {
            println!("Error: {}", err);
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
                println!("exit!");
                break;
            }
            Ok(1) => {
                println!("add block");
                let mut transactions: Vec<String> = Vec::new();
                let die = Uniform::new_inclusive(1, 100);
                let num = die.unwrap().sample(&mut rng);
                for i in 0..num {
                    transactions.push(format!("transaction {}", i));
                }
                let _ = blockchain.add_block(transactions);
                println!("Block added");
            }
            Ok(2) => {
                println!("show all blocks");
                let blockchain_iter = blockchain.iter();
                blockchain_iter.for_each(|block| {
                    println!("timestamp: {}", block.header.timestamp);
                    println!(
                        "previous hash: {}",
                        bytes_to_hex_string(&block.header.prev_hash)
                    );
                    println!("current hash: {}", bytes_to_hex_string(&block.hash));
                    println!("Nonce: {}", block.header.nonce);
                    println!("transactions: {:?}", block.transactions);
                });
            }
            _ => {}
        }
    }
}
fn show() {
    println!("Simple Block Chain");
    println!("1. Add block");
    println!("2. Show all blocks");
    println!("0. Exit");
    print!(">> ");
}
