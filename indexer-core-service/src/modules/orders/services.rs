use crate::utils::{fetch_transaction_status, TransactionDetails};

pub async fn get_orders(hash: &str) -> Result<TransactionDetails, Box<dyn std::error::Error>> {
    match fetch_transaction_status(hash).await {
        Ok(tx) => Ok(tx),
        Err(e) => {
            eprintln!("Error fetching transaction status: {}", e);
            Err(e) // or wrap into a custom error
        }
    }
}