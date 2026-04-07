use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionTrace {
    pub hash: String,
    pub block_number: u64,
    pub gas_used: u64,
    pub status: bool,
    // Add more fields as needed
}

impl TransactionTrace {
    pub fn new(hash: String) -> Self {
        Self {
            hash,
            block_number: 0,
            gas_used: 0,
            status: false,
        }
    }
}
