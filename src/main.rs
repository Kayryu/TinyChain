extern crate ctrlc;

mod block;
mod transaction;

use block::Block;
use transaction::Transaction;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// A Minter that create a block
trait Minter {
    fn mint(&self, trans: Vec<Transaction>) -> Self;
    //fn proof(&self, hash: u64) -> u64;
}

// implement minter for Block
impl Minter for Block {
    fn mint(&self, trans: Vec<Transaction>) -> Self {

        Self {
            number: self.number + 1,
            // TODO
            timestamp: 0,
            // TODO
            parent_hash: 0,
            proof: 0,
            transactions : trans,
        }
    }
}

fn mock_transactions(len: u32) -> Vec<Transaction> {
    //(0..len).map(|_| Transaction::default()).collect().into()
    vec![Transaction::default(), Transaction::default() ]
}

fn main() {
    println!("Wellcome tiny chian! \n Waiting for Ctrl-C...");

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let root = Block::default();
    println!("{:?}", root);
    let mut parent = root;
    while running.load(Ordering::SeqCst) {
        let next = parent.mint(mock_transactions(4));
        println!("mint new block {:?}", next);
        parent = next;
        std::thread::sleep_ms(2_000);
    }
    println!("Byby....");
}