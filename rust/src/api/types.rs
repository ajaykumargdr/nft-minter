use serde::{Deserialize, Serialize};

// Todo : remove unused derives

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
    pub price: f64, // Todo : add more fields like expiration
}

#[allow(unused)] // Todo : remove
#[derive(Debug, Deserialize)]
pub struct QueryParams {
    #[serde(rename = "code")]
    pub auth_code: String,
}

#[derive(Clone)]
pub struct User {
    pub id: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}