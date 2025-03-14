use serde::Deserialize;

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
