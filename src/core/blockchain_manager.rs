use super::blockchain::Blockchain;
use bincode::{deserialize, serialize};
use sled::{Db, Error, open};

pub struct BlockchainManager {
    db: Db,
    pub blockchain: Blockchain,
}

/// Manages blockchain operations including persistence and retrieval
///
/// The `BlockchainManager` struct provides functionality to:
/// - Load a blockchain from disk
/// - Save blockchain state to disk
/// - Access the current blockchain state
/// Creates a new `BlockchainManager` instance
///
/// # Arguments
///
/// * `db_path` - A string slice that holds the path to the database file
///
/// # Returns
///
/// * `Result<Self, Error>` - A new BlockchainManager instance if successful, or an Error if creation fails
///
/// # Note
///
/// If no existing blockchain is found in the database or if deserialization fails,
/// a new blockchain with difficulty level 4 will be created.

/// Returns a clone of the current blockchain
///
/// # Returns
///
/// * `Blockchain` - A copy of the current blockchain state

/// Saves the current blockchain state to disk
///
/// # Returns
///
/// * `Result<(), Error>` - Ok(()) if save is successful, Error otherwise
///
/// # Note
///
/// This method serializes the blockchain and performs a database flush operation
/// to ensure data persistence
impl BlockchainManager {
    pub fn new(db_path: &str) -> Result<Self, Error> {
        let db = open(db_path)?;
        let blockchain = match db.get("blockchain")? {
            Some(data) => match deserialize(&data) {
                Ok(chain) => chain,
                Err(_) => Blockchain::new(4),
            },
            None => Blockchain::new(4),
        };
        println!(
            "Blockchain loaded from storage. Current block height: {}",
            blockchain.chain.len()
        );
        Ok(Self { db, blockchain })
    }

    pub fn get_blockchain(&self) -> Blockchain {
        self.blockchain.clone()
    }

    pub fn save(&self) -> Result<(), Error> {
        let serialized = match serialize(&self.blockchain) {
            Ok(data) => data,
            Err(_) => return Err(Error::Unsupported("Serialization failed".to_string())),
        };
        self.db.insert("blockchain", serialized)?;
        let _ = self.db.flush();
        println!(
            "Blockchain saved successfully. Total blocks: {}",
            self.blockchain.chain.len()
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_blockchain_manager_new() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().to_str().unwrap();

        let manager = BlockchainManager::new(db_path).unwrap();
        assert_eq!(manager.get_blockchain().chain.len(), 1); // Genesis block
    }

    #[test]
    fn test_blockchain_manager_save_and_load() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().to_str().unwrap();

        // Create and save blockchain
        let mut manager1 = BlockchainManager::new(db_path).unwrap();
        let mut chain = manager1.get_blockchain();
        let _ = chain.add_block(vec!["Test data".to_string()]);
        manager1.blockchain = chain;
        manager1.save().unwrap();

        // Load and verify
        let manager2 = BlockchainManager::new(db_path).unwrap();
        assert_eq!(manager2.get_blockchain().chain.len(), 2);
    }

    #[test]
    fn test_blockchain_manager_invalid_path() {
        let result = BlockchainManager::new("invalid_path");
        assert!(result.is_err());
    }

    #[test]
    fn test_blockchain_manager_drop() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().to_str().unwrap();

        {
            let mut manager = BlockchainManager::new(db_path).unwrap();
            let mut chain = manager.get_blockchain();
            let _ = chain.add_block(vec!["Drop test".to_string()]);
            manager.blockchain = chain;
        } // manager gets dropped here

        let new_manager = BlockchainManager::new(db_path).unwrap();
        assert_eq!(new_manager.get_blockchain().chain.len(), 2);
    }
}
