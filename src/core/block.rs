use thiserror::Error;
use chrono::Utc;
use sha2::{Sha256, Digest};

const MAX_TRANSACTIONS_PER_BLOCK: u8 = 20;

#[derive(Debug, Error)]
pub enum BlockError {
    #[error("Block contains more than {} transactions", MAX_TRANSACTIONS_PER_BLOCK)]
    TooManyTransactionsInBlock,
}

#[derive(Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub previous_hash: [u8; 32], // Make sure corresponds with hash function output 
    pub transactions: Vec<u8>, // change to Vec<Transaction> after making the Transaction type 
}

impl Block {
    // constructor like function that require blocks to have an index, timestamp, and previous hash
    pub fn new(index: u64, previous_hash: [u8; 32], transactions: Vec<u8>) -> Block {
        Block {
            index: index,
            timestamp: Utc::now().timestamp() as u64,
            previous_hash: previous_hash,
            transactions: transactions
        }
    }

    // Get the hash of a block
    pub fn hash(&self) -> Result<[u8; 32], BlockError> {
        if self.transactions.len() > MAX_TRANSACTIONS_PER_BLOCK.into() {
            return Err(BlockError::TooManyTransactionsInBlock);
        }

        let mut hasher = Sha256::new();
        hasher.update(self.index.to_be_bytes());
        hasher.update(self.timestamp.to_be_bytes());
        hasher.update(&self.previous_hash);
        for tx in self.transactions.iter() {
            hasher.update(tx.to_be_bytes());
        }

        Ok(hasher.finalize().into())
    }
}