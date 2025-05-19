use super::Result;
use alloy::signers::local::PrivateKeySigner;

pub fn create_eth_account() -> Result<PrivateKeySigner> {
    Ok(PrivateKeySigner::random())
}
