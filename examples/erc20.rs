//! Example on how to interact with a deployed `Erc20Workshop` contract using defaults.
//! This example uses ethers-rs to instantiate the contract using a Solidity ABI.

use dotenv::dotenv;
use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, U256},
};
use eyre::eyre;
use std::str::FromStr;
use std::sync::Arc;

/// Your private key.
const PRIV_KEY: &str = "PRIV_KEY";

/// Stylus RPC endpoint url.
const RPC_URL: &str = "RPC_URL";

/// Deployed pragram address.
const STYLUS_CONTRACT_ADDRESS: &str = "STYLUS_CONTRACT_ADDRESS";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv().ok();
    let priv_key = std::env::var(PRIV_KEY).map_err(|_| eyre!("No {} env var set", PRIV_KEY))?;
    let rpc_url = std::env::var(RPC_URL).map_err(|_| eyre!("No {} env var set", RPC_URL))?;
    let contract_address = std::env::var(STYLUS_CONTRACT_ADDRESS)
        .map_err(|_| eyre!("No {} env var set", STYLUS_CONTRACT_ADDRESS))?;
    abigen!(
        Erc20Workshop,
        r#"[
            function totalSupply() external view returns (uint256 totalSupply)
            function balanceOf(address account) external view returns (uint256 balance)
            function transfer(address recipient, uint256 amount) external returns (bool)
            function allowance(address owner, address spender) external view returns (uint256 allowance)
            function approve(address spender, uint256 amount) external returns (bool)
            function transferFrom(address sender, address recipient, uint256 amount) external returns (bool)

            function burn(uint256 amount) external
            function burnFrom(address account, uint256 amount) external

            function name() external view returns (string name)
            function symbol() external view returns (string symbol)
            function decimals() external view returns (uint8 decimals)

            function mint(address account, uint256 amount) external

            function owner() external view returns (address owner)
            function transferOwnership(address newOwner) external
            function renounceOwnership() external
        ]"#
    );

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let address: Address = contract_address.parse()?;

    let wallet = LocalWallet::from_str(&priv_key)?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.clone().with_chain_id(chain_id),
    ));

    let token = Erc20Workshop::new(address, client);

    println!("Wallet address = {:?}\n", wallet.address());

    // Read token metadata.
    let name: String = token.name().call().await?;
    println!("Token name = {:?}\n", name);

    // Read initial balance.
    let initial_balance: U256 = token.balance_of(wallet.address()).call().await?;
    println!("Initial balance = {:?}\n", initial_balance);

    // Mint tokens.
    let amount = U256::from(1000000000);
    let pending = token.mint(wallet.address(), amount);
    if let Some(receipt) = pending.send().await?.await? {
        println!("Receipt = {:?}\n", receipt);
    }
    println!("Successfully minted tokens via a tx\n");

    // Read final balance.
    let final_balance: U256 = token.balance_of(wallet.address()).call().await?;
    println!("Final balance = {:?}\n", final_balance);

    assert_eq!(final_balance, initial_balance + amount);

    Ok(())
}
