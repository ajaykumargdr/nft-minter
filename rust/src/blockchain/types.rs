use serde::Serialize;

#[derive(Serialize)]
pub struct Metadata {
    pub owner_address: String,
    pub token_uri: String,
}
