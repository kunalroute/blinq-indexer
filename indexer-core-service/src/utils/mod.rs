use serde_json::Value;
use reqwest::Client;

const CCTP_API_URL: &str = "https://usdc.range.org/api/payments?...";
const RELAY_API_URL: &str = "https://api.relay.link/requests/v2";

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionDetails {
    source_token: String,
    source_amount: f64,
    source_symbol: String,
    dest_token: String,
    dest_amount: f64,
    dest_symbol: String,
    fee: Option<FeeDetails>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeDetails {
    gas: Option<f64>,
    fixed: Option<f64>,
    price: Option<f64>,
}

/// Fetches the transaction status from the CCTP API and parses the response into a common format.
///
/// # Returns
/// A `Result` containing `TransactionDetails` on success or an error on failure.
///
/// # Example
/// ```
/// let result = fetch_cctp_transaction_status().await;
/// match result {
///     Ok(details) => println!("{:?}", details),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
async fn fetch_cctp_transaction_status() -> Result<TransactionDetails, Box<dyn std::error::Error>> {
    let client = Client::new();
    let response: Value = client.get(CCTP_API_URL).send().await?.json().await?;

    let resource = &response["resources"][0];
    let bridge_info = &resource["bridge_info"];

    Ok(TransactionDetails {
        source_token: resource["sender_entity"].as_str().unwrap_or_default().to_string(),
        source_amount: resource["sender_amount"].as_f64().unwrap_or_default(),
        source_symbol: resource["sender_symbol"].as_str().unwrap_or_default().to_string(),
        dest_token: resource["receiver_entity"].as_str().unwrap_or_default().to_string(),
        dest_amount: resource["usd"].as_f64().unwrap_or_default(),
        dest_symbol: resource["sender_symbol"].as_str().unwrap_or_default().to_string(),
        fee: None, // CCTP response does not include fee details
    })
}

/// Fetches the transaction status from the Relay API and parses the response into a common format.
///
/// # Parameters
/// - `hash`: The transaction hash to include in the API URL.
///
/// # Returns
/// A `Result` containing `TransactionDetails` on success or an error on failure.
///
/// # Example
/// ```
/// let result = fetch_relay_transaction_status("some_hash").await;
/// match result {
///     Ok(details) => println!("{:?}", details),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
async fn fetch_relay_transaction_status(hash: &str) -> Result<TransactionDetails, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("{}?hash={}", RELAY_API_URL, hash);
    let response: Value = client.get(&url).send().await?.json().await?;

    let request = &response["requests"][0];
    let metadata = &request["data"]["metadata"];
    let fees = &request["data"]["feesUsd"];

    Ok(TransactionDetails {
        source_token: metadata["currencyIn"]["currency"]["address"].as_str().unwrap_or_default().to_string(),
        source_amount: metadata["currencyIn"]["amountFormatted"].as_str().unwrap_or_default().parse().unwrap_or_default(),
        source_symbol: metadata["currencyIn"]["currency"]["symbol"].as_str().unwrap_or_default().to_string(),
        dest_token: metadata["currencyOut"]["currency"]["address"].as_str().unwrap_or_default().to_string(),
        dest_amount: metadata["currencyOut"]["amountFormatted"].as_str().unwrap_or_default().parse().unwrap_or_default(),
        dest_symbol: metadata["currencyOut"]["currency"]["symbol"].as_str().unwrap_or_default().to_string(),
        fee: Some(FeeDetails {
            gas: fees["gas"].as_str().unwrap_or_default().parse().ok(),
            fixed: fees["fixed"].as_str().unwrap_or_default().parse().ok(),
            price: fees["price"].as_str().unwrap_or_default().parse().ok(),
        }),
    })
}

/// Fetches the transaction status by calling both the CCTP and Relay APIs.
/// Returns the response from the first successful API call. If both succeed, returns any one.
///
/// # Parameters
/// - `hash`: The transaction hash to include in the Relay API URL.
///
/// # Returns
/// A `Result` containing `TransactionDetails` on success or an error if both API calls fail.
///
/// # Example
/// ```
/// let result = fetch_transaction_status("some_hash").await;
/// match result {
///     Ok(details) => println!("{:?}", details),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
pub async fn fetch_transaction_status(hash: &str) -> Result<TransactionDetails, Box<dyn std::error::Error>> {
    let cctp_result = fetch_cctp_transaction_status().await;
    if cctp_result.is_ok() {
        return cctp_result;
    }

    let relay_result = fetch_relay_transaction_status(hash).await;
    if relay_result.is_ok() {
        return relay_result;
    }

    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Both API calls failed",
    )))
}
