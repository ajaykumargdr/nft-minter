use GenesisToken::GenesisTokenInstance;
use actix_web::{get, web};
use alloy::{
    network::EthereumWallet,
    primitives::{Address, U256},
    providers::{
        self, ProviderBuilder,
        fillers::{FillProvider, JoinFill, WalletFiller},
        utils::JoinedRecommendedFillers,
    },
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

type GTKProvider = FillProvider<
    JoinFill<JoinedRecommendedFillers, WalletFiller<EthereumWallet>>,
    providers::RootProvider,
>;

async fn new_contract() -> Result<GenesisTokenInstance<(), GTKProvider>> {
    println!("creating new contract instance...");
    // let alchemy_sepolia_url = env::var("ALCHEMY_SEPOLIA_URL")?;
    let nft_contract_address = env::var("NFT_CONTRACT_ADDRESS")?;

    let testing_network_url = env::var("TESTING_NETWORK_URL")?;
    let owner_private_address = env::var("OWNER_PRIVATE_ADDRESS")?; // only owner can mint nfts
    let testing_account1_private_key: PrivateKeySigner =
        env::var("TESTING_ACCOUNT1_PRIVATE_KEY")?.parse()?;

    // adding addresses for signing
    let default_signer: PrivateKeySigner = owner_private_address.parse()?;
    let mut wallet = EthereumWallet::from(default_signer);
    wallet.register_signer(testing_account1_private_key.clone());

    let provider = ProviderBuilder::new()
        .wallet(wallet.clone())
        .on_http(testing_network_url.parse()?);

    // Create a contract instance.
    Ok(GenesisToken::new(
        Address::from_str(&nft_contract_address)?,
        provider,
    ))
}

// This struct represents state
struct AppState {
    contract: GenesisTokenInstance<(), GTKProvider>,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    data.contract.name().call().await.unwrap()._0
}

#[actix_web::test]
async fn test_actix() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    dotenv().ok();
    let contract = new_contract().await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                contract: contract.clone(),
            }))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let contract = new_contract().await?;
    let contract_owner = Address::from_str(&env::var("INITIAL_OWNER")?)?;
    let testing_account1_private_key: PrivateKeySigner =
        env::var("TESTING_ACCOUNT1_PRIVATE_KEY")?.parse()?;

    // calling a contract method
    let contract_name = contract.name().call().await?._0;
    println!("Contract name: {contract_name}");

    // calling contract to mint nft
    let token_id = U256::from(5);
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
