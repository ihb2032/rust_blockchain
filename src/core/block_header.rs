use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BlockHeader {
    pub timestamp: u64,
    pub prev_hash: Vec<u8>,
    pub nonce: u64,
    pub difficulty: u32,
}

/// Represents the header of a block in the blockchain.
///
/// This struct contains essential information for each block, including:
/// - `timestamp`: The time at which the block was created, measured in seconds since the UNIX epoch.
/// - `prev_hash`: A vector of bytes representing the hash of the previous block in the chain.
/// - `nonce`: A number used for mining, initialized to 0.
/// - `difficulty`: The difficulty level for mining the block.
///
/// The `new` function initializes a new `BlockHeader` with the provided previous hash and difficulty,
/// setting the timestamp to the current time.
impl BlockHeader {
    pub fn new(prev_hash: Vec<u8>, difficulty: u32) -> Self {
        let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };
        Self {
            timestamp,
            prev_hash,
            nonce: 0,
            difficulty,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_header_creation() {
        let prev_hash = vec![0u8; 32];
        let difficulty = 2;
        let block_header = BlockHeader::new(prev_hash.clone(), difficulty);

        assert_eq!(block_header.prev_hash, prev_hash);
        assert_eq!(block_header.difficulty, difficulty);
        assert!(block_header.timestamp > 0);
        assert_eq!(block_header.nonce, 0);
    }

    #[test]
    fn test_block_header_difficulty() {
        let prev_hash = vec![1u8; 32];
        let difficulty = 5;
        let block_header = BlockHeader::new(prev_hash.clone(), difficulty);

        assert_eq!(block_header.difficulty, difficulty);
    }

    // New tests added below
    #[test]
    fn test_block_header_nonce_initialization() {
        let prev_hash = vec![2u8; 32];
        let difficulty = 3;
        let block_header = BlockHeader::new(prev_hash.clone(), difficulty);

        assert_eq!(block_header.nonce, 0);
    }

    #[test]
    fn test_block_header_timestamp() {
        let prev_hash = vec![3u8; 32];
        let difficulty = 4;
        let block_header = BlockHeader::new(prev_hash.clone(), difficulty);

        assert!(block_header.timestamp > 0);
    }
}
