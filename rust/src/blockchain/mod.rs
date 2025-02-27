use super::Result;
use GenesisToken::GenesisTokenInstance;

use alloy::{
    network::EthereumWallet,
    primitives::Address,
    providers::{
        ProviderBuilder, RootProvider,
        fillers::{FillProvider, JoinFill, WalletFiller},
        utils::JoinedRecommendedFillers,
    },
    signers::local::PrivateKeySigner,
    sol,
};
use std::{env, str::FromStr};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    GenesisToken,
    "../hardhat/artifacts/contracts/GenesisToken.sol/GenesisToken.json"
);

type GTKProvider =
    FillProvider<JoinFill<JoinedRecommendedFillers, WalletFiller<EthereumWallet>>, RootProvider>;

#[derive(Clone)]
pub struct GTKContract {
    contract: GenesisTokenInstance<(), GTKProvider>,
}

impl GTKContract {
    pub async fn new() -> Result<Self> {
        println!("creating new contract instance...");
        // let alchemy_sepolia_url = env::var("ALCHEMY_SEPOLIA_URL")?;
        let nft_contract_address = env::var("NFT_CONTRACT_ADDRESS")?;

        let testing_network_url = env::var("TESTING_NETWORK_URL")?;
        let owner_private_address = env::var("OWNER_PRIVATE_ADDRESS")?; // only owner can mint nfts
        let testing_account1_private_key: PrivateKeySigner =
            env::var("TESTING_ACCOUNT1_PRIVATE_KEY")?.parse()?;

        // adding addresses for signing
        let default_signer: PrivateKeySigner = owner_private_address.parse()?;
        // Todo - check for dynamic adding of signers
        let mut wallet = EthereumWallet::from(default_signer.clone());
        wallet.register_signer(testing_account1_private_key);

        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .on_http(testing_network_url.parse()?);

        let contract = GenesisToken::new(Address::from_str(&nft_contract_address)?, provider);

        // Create a contract instance.
        Ok(Self { contract })
    }

    pub async fn contract_name(&self) -> Result<String> {
        Ok(self.contract.name().call().await?._0)
    }
}

#[tokio::test]
async fn test_contract() -> Result<()> {
    dotenv::dotenv().ok();

    let contract = GTKContract::new().await?.contract;
    let contract_owner = Address::from_str(&env::var("INITIAL_OWNER")?)?;
    let testing_account1_private_key: PrivateKeySigner =
        env::var("TESTING_ACCOUNT1_PRIVATE_KEY")?.parse()?;

    // calling a contract method
    let contract_name = contract.name().call().await?._0;
    println!("Contract name: {contract_name}");

    // calling contract to mint nft
    let token_id = alloy::primitives::U256::from(5);
    let token_owner = testing_account1_private_key.address();
    let _mint= contract.safeMint(
        token_owner,
        token_id,
        String::from("https://gateway.pinata.cloud/ipfs/bafkreifsbhqh4lb3k7n3usnkecxdzv6ef4bahft5sbzru6tc3qebruejcq"),
    ).from(contract_owner).send().await?.watch().await?;

    println!("{:?}", _mint);

    let owner = contract.ownerOf(token_id).call().await?._0;
    println!("Owner: {owner}");

    assert_eq!(owner, token_owner);

    // example of transferring nft

    println!(
        "\nTransferring 0 from {:?} to {:?}",
        token_owner, contract_owner
    );
    contract
        .safeTransferFrom_0(token_owner, contract_owner, token_id)
        .from(token_owner)
        .send()
        .await?
        .watch()
        .await?;

    let owner = contract.ownerOf(token_id).call().await?._0;
    println!("Owner: {owner}");

    Ok(())
}
