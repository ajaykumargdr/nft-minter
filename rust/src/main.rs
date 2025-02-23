use alloy::{
    network::EthereumWallet,
    primitives::{Address, U256, address},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    sol,
};
use dotenv::dotenv;
use std::{env, str::FromStr};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// Codegen from ABI file to interact with the contract.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    GenesisToken,
    "../hardhat/artifacts/contracts/GenesisToken.sol/GenesisToken.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let alchemy_sepolia_url = env::var("ALCHEMY_SEPOLIA_URL")?;
    let nft_contract_address = env::var("NFT_CONTRACT_ADDRESS")?;
    let contract_owner = Address::from_str(&env::var("INITIAL_OWNER")?)?;
    let owner_private_address = env::var("OWNER_PRIVATE_ADDRESS")?;

    let signer: PrivateKeySigner = owner_private_address.parse()?;
    let wallet = EthereumWallet::from(signer);

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .on_http(alchemy_sepolia_url.parse()?);

    // Create a contract instance.
    let contract = GenesisToken::new(Address::from_str(&nft_contract_address)?, provider);
    let contract_name = contract.name().call().await?._0;
    println!("Contract name: {contract_name}");

    let token_id = U256::from(2);
    let token_owner = address!("0xCef36f07A7f31456142aAa82DC11DA26a8085583");
    let _mint= contract.safeMint(   
        token_owner, 
        token_id,
        String::from("https://gateway.pinata.cloud/ipfs/bafkreifsbhqh4lb3k7n3usnkecxdzv6ef4bahft5sbzru6tc3qebruejcq"),
    ).from(contract_owner).send().await;

    let owner = contract.ownerOf(token_id).call().await?._0;
    println!("Owner: {owner}");

    assert_eq!(owner, token_owner);
    Ok(())
}
