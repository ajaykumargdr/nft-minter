use super::Result;
use GenesisToken::GenesisTokenInstance;

use alloy::{
    consensus::{SignableTransaction, TxLegacy},
    network::{EthereumWallet, TxSigner},
    primitives::{Address, TxKind, U256},
    providers::{
        Provider, ProviderBuilder, RootProvider,
        fillers::{FillProvider, JoinFill, WalletFiller},
        utils::JoinedRecommendedFillers,
    },
    signers::local::PrivateKeySigner,
    sol,
};
use std::{env, str::FromStr};

mod types;
pub use types::*;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    GenesisToken,
    "../artifacts/contracts/GenesisToken.sol/GenesisToken.json"
);

type GTKProvider =
    FillProvider<JoinFill<JoinedRecommendedFillers, WalletFiller<EthereumWallet>>, RootProvider>;

#[derive(Clone)]
pub struct GTKContract {
    contract: GenesisTokenInstance<(), GTKProvider>,
    owner_address: Address,
}

impl GTKContract {
    pub async fn new() -> Result<Self> {
        let nft_contract_address = env::var("NFT_CONTRACT_ADDRESS")?;
        let url = env::var("NETWORK_URL")?;
        let owner_private_key: PrivateKeySigner = env::var("OWNER_PRIVATE_KEY")?.parse()?; // only owner can mint nfts

        // adding addresses for signing
        let wallet = EthereumWallet::from(owner_private_key.clone());
        let provider = ProviderBuilder::new().wallet(wallet).on_http(url.parse()?);

        let contract = GenesisToken::new(Address::from_str(&nft_contract_address)?, provider);

        Ok(Self {
            contract,
            owner_address: owner_private_key.address(),
        })
    }

    pub async fn contract_name(&self) -> Result<String> {
        Ok(self.contract.name().call().await?._0)
    }

    pub async fn mint_nft(&self, to: &str, token_id: usize, token_uri: &str) -> Result<()> {
        self.contract
            .safeMint(
                Address::from_str(to)?,
                U256::from(token_id),
                token_uri.to_string(),
            )
            .from(self.owner_address)
            .send()
            .await?
            .watch()
            .await?;

        Ok(())
    }

    pub async fn owner_of_token(&self, id: usize) -> Result<String> {
        Ok(self
            .contract
            .ownerOf(U256::from(id))
            .call()
            .await?
            ._0
            .to_string())
    }

    pub async fn transfer_nft(&self, from: &str, to: &str, token_id: usize) -> Result<()> {
        // Todo : use keystore and wallet
        let signer = from.parse::<PrivateKeySigner>()?;
        let signer_address = signer.address();

        // Todo : implement transaction using eip-1559
        let data = self
            .contract
            .safeTransferFrom_0(signer_address, Address::from_str(to)?, U256::from(token_id))
            .calldata()
            .clone();

        let provider = self.contract.provider();

        let mut tx = TxLegacy {
            chain_id: Some(provider.get_chain_id().await?),
            nonce: provider.get_transaction_count(signer_address).await?,
            gas_price: provider.get_gas_price().await?,
            gas_limit: 80000,
            to: TxKind::Call(self.contract.address().clone()),
            input: data.clone(),
            value: Default::default(),
        };

        let signature = signer.sign_transaction(&mut tx).await?;

        let mut out = Vec::new();
        tx.into_signed(signature).rlp_encode(&mut out);

        let _pending_tx = self
            .contract
            .provider()
            .send_raw_transaction(&out)
            .await
            .unwrap()
            .watch()
            .await?;

        Ok(())
    }

    pub async fn get_metadata(&self, token_id: usize) -> Result<Metadata> {
        let owner_address = self
            .contract
            .ownerOf(U256::from(token_id))
            .call()
            .await?
            ._0
            .to_string();

        let token_uri = self
            .contract
            .tokenURI(U256::from(token_id))
            .call()
            .await?
            ._0;

        Ok(Metadata {
            owner_address,
            token_uri,
        })
    }
}

#[tokio::test]
async fn test_contract() -> Result<()> {
    use alloy::{
        consensus::{SignableTransaction, TxLegacy},
        network::TxSigner,
        primitives::TxKind,
        providers::Provider,
    };

    dotenv::dotenv().ok();

    let contract = GTKContract::new().await.unwrap().contract;
    let provider = contract.provider();

    let contract_owner = Address::from_str(&env::var("INITIAL_OWNER")?)?;

    let test_acc1: PrivateKeySigner = env::var("TESTING_ACCOUNT1_PRIVATE_KEY")?.parse()?;

    // calling a contract method
    let contract_name = contract.name().call().await?._0;
    println!("Contract name: {contract_name}");

    // calling contract to mint nft
    let ptr = Box::into_raw(Box::new(123));
    println!("token: {}", ptr as usize);
    let token_id = alloy::primitives::U256::from(ptr as usize);

    let _mint= contract.safeMint(
        test_acc1.address(),
            token_id,
            String::from("https://gateway.pinata.cloud/ipfs/bafkreifsbhqh4lb3k7n3usnkecxdzv6ef4bahft5sbzru6tc3qebruejcq"),
        ).from(contract_owner).send().await?.watch().await?;

    let owner = contract.ownerOf(token_id).call().await?._0;
    println!("Owner of {token_id}: {owner}");

    assert_eq!(owner, test_acc1.address());

    println!(
        "\nTransferring {token_id} from {:?} to {:?}",
        test_acc1.address(),
        contract_owner
    );

    let data = contract
        .safeTransferFrom_0(test_acc1.address(), contract_owner, token_id)
        .calldata()
        .clone();

    let mut tx = TxLegacy {
        chain_id: Some(provider.get_chain_id().await?),
        nonce: provider.get_transaction_count(test_acc1.address()).await?,
        gas_price: provider.get_gas_price().await?,
        gas_limit: 80000,
        to: TxKind::Call(contract.address().clone()),
        input: data.clone(),
        value: Default::default(),
    };

    let signature = test_acc1.sign_transaction(&mut tx).await?;

    let mut out = Vec::new();
    tx.into_signed(signature).rlp_encode(&mut out);

    let _pending_tx = contract
        .provider()
        .send_raw_transaction(&out)
        .await
        .unwrap()
        .watch()
        .await?;

    let owner = contract.ownerOf(token_id).call().await?._0;
    println!("Owner({token_id}): {owner}");

    assert_eq!(owner, contract_owner);

    Ok(())
}
