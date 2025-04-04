use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct MintInfo {
    pub to: String,
    pub token_id: usize,
    pub token_uri: String,
}

#[derive(Debug, Deserialize)]
pub struct TransferInfo {
    pub from: String,
    pub to: String,
    pub token_id: usize,
}

#[allow(unused)] // Todo : remove
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListingInfo {
    pub token_id: usize,
    pub price: f64
    // Todo : add more fields like expiration
}
