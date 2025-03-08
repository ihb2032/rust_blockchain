use super::block_header::BlockHeader;
use crate::utils::hash::{bytes_to_hex_string, hex_string_to_bytes};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<String>,
    pub hash: Vec<u8>,
}

/// Represents a block in the blockchain.
///
/// The `Block` struct contains a header, a list of transactions, and a hash.
/// It provides methods to create a new block, calculate its hash, and mine it.
///
/// # Methods
///
/// - `new(prev_hash_hex: String, transactions: Vec<String>, difficulty: u32) -> Self`
///   Creates a new block with the given previous hash, transactions, and mining difficulty.
///
/// - `calculate_hash(&self) -> Vec<u8>`
///   Calculates the hash of the block based on its header and transactions.
///
/// - `mine(&mut self)`
///   Mines the block by finding a valid hash that meets the specified difficulty.
impl Block {
    pub fn new(prev_hash_hex: String, transactions: Vec<String>, difficulty: u32) -> Self {
        let prev_hash = hex_string_to_bytes(&prev_hash_hex);
        let header = BlockHeader::new(prev_hash, difficulty);
        let mut block = Self {
            header,
            transactions,
            hash: vec![],
        };
        block.mine();
        block
    }

    fn calculate_hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        let prev_hash_hex = bytes_to_hex_string(&self.header.prev_hash);
        let data = format!(
            "{}{}{}{}",
            self.header.timestamp,
            prev_hash_hex,
            self.header.nonce,
            self.transactions.join("")
        );
        hasher.update(data.as_bytes());
        hasher.finalize().to_vec()
    }

    fn mine(&mut self) {
        let target_prefix = vec![0u8; (self.header.difficulty / 8) as usize];
        let remaining_bits = self.header.difficulty % 8;
        let last_byte_mask = if remaining_bits > 0 {
            0xFF >> remaining_bits
        } else {
            0
        };

        loop {
            self.hash = self.calculate_hash();

            let mut matches = self.hash.starts_with(&target_prefix);
            if matches && remaining_bits > 0 {
                matches = self.hash[target_prefix.len()] <= last_byte_mask;
            }

            if matches {
                break;
            }
            self.header.nonce += 1;
        }

        println!("Block mined: {}", bytes_to_hex_string(&self.hash));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_creation() {
        let prev_hash =
            "0000000000000000000000000000000000000000000000000000000000000000".to_string();
        let transactions = vec!["tx1".to_string(), "tx2".to_string()];
        let difficulty = 16;

        let block = Block::new(prev_hash, transactions.clone(), difficulty);

        assert_eq!(block.transactions, transactions);
        assert!(!block.hash.is_empty());
    }

    #[test]
    fn test_calculate_hash() {
        let prev_hash =
            "0000000000000000000000000000000000000000000000000000000000000000".to_string();
        let transactions = vec!["tx1".to_string()];
        let difficulty = 16;

        let block = Block::new(prev_hash, transactions, difficulty);
        let hash = block.calculate_hash();

        assert_eq!(hash.len(), 32); // Sha256 produces a 32-byte hash
    }

    // New tests added
    #[test]
    fn test_mine_valid_hash() {
        let prev_hash =
            "0000000000000000000000000000000000000000000000000000000000000000".to_string();
        let transactions = vec!["tx1".to_string()];
        let difficulty = 16;

        let mut block = Block::new(prev_hash, transactions, difficulty);
        block.mine();

        assert!(
            block
                .hash
                .starts_with(&vec![0u8; (difficulty / 8) as usize])
        ); // Check prefix
    }
}
