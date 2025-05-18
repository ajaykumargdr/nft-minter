use super::Result;
use ssss::SsssConfig;

pub fn split_secret(secret: &[u8]) -> Result<Vec<String>> {
    let mut config = SsssConfig::default();
    config.set_num_shares(3);
    config.set_threshold(3);

    Ok(ssss::gen_shares(&config, &secret)?)
}

pub fn recover_secret(shares: &[String]) -> Result<Vec<u8>> {
    Ok(ssss::unlock(&shares)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_share() {
        let secret = b"secret";
        let shares = split_secret(secret).unwrap();
        let recovered_secret = recover_secret(&shares).unwrap();
        assert_eq!(recovered_secret, secret);
    }

    #[test]
    fn test_secret_share_fail() {
        let secret = b"secret";
        let shares = split_secret(secret).unwrap();
        let recovered_secret = recover_secret(&shares[..2]).unwrap();
        assert_ne!(recovered_secret, secret);
    }
}
