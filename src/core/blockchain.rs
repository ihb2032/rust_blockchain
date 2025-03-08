use super::block::Block;
use crate::utils::hash::bytes_to_hex_string;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: u32,
}

/// A structure representing a blockchain.
///
/// The `Blockchain` struct manages a chain of blocks, allowing for the creation of new blocks
/// and retrieval of the last block in the chain.
///
/// # Methods
///
/// - `new(difficulty: u32) -> Self`: Creates a new instance of `Blockchain` with the specified
///   difficulty level. It initializes the chain and creates the genesis block.
///
/// - `create_genesis_block(&mut self)`: Private method that creates the first block in the
///   blockchain, known as the genesis block, and adds it to the chain.
///
/// - `get_last_block(&self) -> Option<&Block>`: Returns a reference to the last block in the
///   blockchain, or `None` if the chain is empty.
///
/// - `add_block(&mut self, transactions: Vec<String>) -> Result<(), &'static str>`: Adds a new
///   block containing the provided transactions to the blockchain. Returns an error if the
///   blockchain is empty.
impl Blockchain {
    pub fn new(difficulty: u32) -> Self {
        let mut blockchain = Self {
            chain: Vec::new(),
            difficulty,
        };
        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        let genesis_block =
            Block::new("0".repeat(64), vec!["genesis".to_string()], self.difficulty);

        self.chain.push(genesis_block.clone());
        println!("Genesis block initialized.");
        println!("Hash: {}", bytes_to_hex_string(&genesis_block.hash));
        println!("Transactions: {:?}", genesis_block.transactions);
        println!("Nonce: {}", genesis_block.header.nonce);
    }

    pub fn get_last_block(&self) -> Option<&Block> {
        self.chain.last()
    }

    pub fn add_block(&mut self, transactions: Vec<String>) -> Result<(), &'static str> {
        let last_block = self
            .get_last_block()
            .ok_or("Blockchain is empty. Cannot add block.")?;

        let new_block = Block::new(
            bytes_to_hex_string(last_block.hash.as_slice()),
            transactions,
            self.difficulty,
        );

        self.chain.push(new_block);
        Ok(())
    }
}

pub struct BlockchainIterator<'a> {
    blockchain: &'a Blockchain,
    current_index: usize,
}

impl<'a> IntoIterator for &'a Blockchain {
    type Item = &'a Block;
    type IntoIter = BlockchainIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BlockchainIterator {
            blockchain: self,
            current_index: 0,
        }
    }
}

impl<'a> Iterator for BlockchainIterator<'a> {
    type Item = &'a Block;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < self.blockchain.chain.len() {
            let block = &self.blockchain.chain[self.current_index];
            self.current_index += 1;
            Some(block)
        } else {
            None
        }
    }
}

impl Blockchain {
    pub fn iter(&self) -> BlockchainIterator {
        self.into_iter()
    }

    pub fn iter_reverse(&self) -> impl Iterator<Item = &Block> {
        self.chain.iter().rev()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_genesis_block() {
        let blockchain = Blockchain::new(2);
        assert_eq!(blockchain.chain.len(), 1);
        assert_eq!(
            blockchain.chain[0].transactions,
            vec!["genesis".to_string()]
        );
    }

    #[test]
    fn test_add_block() {
        let mut blockchain = Blockchain::new(2);
        let result = blockchain.add_block(vec!["transaction1".to_string()]);
        assert!(result.is_ok());
        assert_eq!(blockchain.chain.len(), 2);
        assert_eq!(
            blockchain.chain[1].transactions,
            vec!["transaction1".to_string()]
        );
    }

    #[test]
    fn test_get_last_block() {
        let mut blockchain = Blockchain::new(2);
        blockchain
            .add_block(vec!["transaction1".to_string()])
            .unwrap();
        let last_block = blockchain.get_last_block().unwrap();
        assert_eq!(last_block.transactions, vec!["transaction1".to_string()]);
    }

    // New tests added
    #[test]
    fn test_empty_blockchain() {
        let blockchain = Blockchain::new(2);
        assert!(blockchain.get_last_block().is_some());
    }

    #[test]
    fn test_add_multiple_blocks() {
        let mut blockchain = Blockchain::new(2);
        blockchain
            .add_block(vec!["transaction1".to_string()])
            .unwrap();
        blockchain
            .add_block(vec!["transaction2".to_string()])
            .unwrap();
        assert_eq!(blockchain.chain.len(), 3);
        assert_eq!(
            blockchain.chain[2].transactions,
            vec!["transaction2".to_string()]
        );
    }

    #[test]
    fn test_iterate_blocks() {
        let mut blockchain = Blockchain::new(2);
        blockchain
            .add_block(vec!["transaction1".to_string()])
            .unwrap();
        blockchain
            .add_block(vec!["transaction2".to_string()])
            .unwrap();

        let mut iter = blockchain.iter();
        assert_eq!(
            iter.next().unwrap().transactions,
            vec!["genesis".to_string()]
        );
        assert_eq!(
            iter.next().unwrap().transactions,
            vec!["transaction1".to_string()]
        );
        assert_eq!(
            iter.next().unwrap().transactions,
            vec!["transaction2".to_string()]
        );
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_iter_reverse() {
        let mut blockchain = Blockchain::new(2);
        blockchain
            .add_block(vec!["transaction1".to_string()])
            .unwrap();
        blockchain
            .add_block(vec!["transaction2".to_string()])
            .unwrap();

        let mut iter = blockchain.iter_reverse();
        assert_eq!(
            iter.next().unwrap().transactions,
            vec!["transaction2".to_string()]
        );
        assert_eq!(
            iter.next().unwrap().transactions,
            vec!["transaction1".to_string()]
        );
        assert_eq!(
            iter.next().unwrap().transactions,
            vec!["genesis".to_string()]
        );
        assert!(iter.next().is_none());
    }
}
