use super::Result;
use alloy::signers::local::PrivateKeySigner;

pub fn create_eth_account() -> Result<PrivateKeySigner> {
    Ok(PrivateKeySigner::random())
}

pub fn create_eth_account_from_bytes(bytes: &[u8]) -> Result<PrivateKeySigner> {
    Ok(PrivateKeySigner::from_slice(bytes)?)
}
