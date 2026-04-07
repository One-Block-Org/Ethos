pub struct EthClient {
    pub rpc_url: String,
}

impl EthClient {
    pub fn new(rpc_url: String) -> Self {
        Self { rpc_url }
    }

    pub async fn get_transaction_trace(&self, tx_hash: &str) -> anyhow::Result<String> {
        // Implementation for debug_traceTransaction will go here
        Ok(format!("Trace for transaction: {}", tx_hash))
    }
}
