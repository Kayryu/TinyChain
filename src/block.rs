/*
block informations, we use it as protobuf.
It not a general block.
*/

use std::time::SystemTime;
use transaction::Transaction;

/// A block, encoded as it is on the block chain.
#[derive(Hash, Default, Debug, Clone, PartialEq)]
pub struct Block {
    // Block number
    pub number: u64,
    // Parent hash
    pub parent_hash: u64,
    // Block timestamp
    pub timestamp: u64,
    // Block proof
    pub proof: u64,
    // Block transactions
    pub transactions: Vec<Transaction>,
}
