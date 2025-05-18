use serde::{Deserialize, Serialize};

use crate::{blockchain::GTKContract, secret_storage::HcpClient};

// Todo : remove unused derives

#[derive(Clone)]
pub struct ActixContext {
    pub contract: GTKContract,
    pub http_client: reqwest::Client,
    pub secret_manager: HcpClient,
}

#[derive(Debug, Deserialize)]
pub struct MintInfo {
    // Todo : make token_id auto-increment
    pub token_id: usize,
    pub token_uri: String,
}

#[derive(Debug, Deserialize)]
pub struct TransferInfo {
    pub from: String,
    pub to: String,
    pub token_id: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListingInfo {
    pub token_id: usize,
    pub price: f64, // Todo : add more fields like expiration
}

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    #[serde(rename = "code")]
    pub auth_code: String,
}

#[derive(Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub key_shares: [String; 2],
    pub wallet_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}
