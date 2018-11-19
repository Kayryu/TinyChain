
// A transaction that inclue sender and receiver address.
#[derive(Hash, Default, Debug, Clone, PartialEq)]
pub struct Transaction {
    // The sender of transaction
    pub from: String,
    // The receiver of transaction
    pub to: String,
    // The value of transaction
    pub value: u64,
}

