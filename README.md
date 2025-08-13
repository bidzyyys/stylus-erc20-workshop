# Stylus ERC-20 Workshop

## Quick Start

Install [Rust](https://www.rust-lang.org/tools/install), and then install the Stylus CLI tool with Cargo

```bash
cargo install --force cargo-stylus cargo-stylus-check
```

Add the `wasm32-unknown-unknown` build target to your Rust compiler:

```
rustup target add wasm32-unknown-unknown
```

You should now have it available as a Cargo subcommand:

```bash
cargo stylus --help
```

Then, clone the repository:

```
git clone https://github.com/bidzyyys/stylus-erc20-workshop.git && cd stylus-erc20-workshop
```

### Testnet Information

All testnet information, including faucets and RPC endpoints can be found [here](https://docs.arbitrum.io/stylus/reference/testnet-information).

### ABI Export

You can export the Solidity ABI for your program by using the `cargo stylus` tool as follows:

```bash
cargo stylus export-abi
```

Exporting ABIs uses a feature that is enabled by default in your Cargo.toml:

```toml
[features]
# stylus-sdk/export-abi will be enabled automatically.
export-abi = ["openzeppelin-stylus/export-abi"]
```

## Deploying

You can use the `cargo stylus` command to also deploy your program to the Stylus testnet. We can use the tool to first check
our program compiles to valid WASM for Stylus and will succeed a deployment onchain without transacting. By default, this will use the Stylus testnet public RPC endpoint. See here for [Stylus testnet information](https://docs.arbitrum.io/stylus/reference/testnet-information)

```bash
cargo stylus check -e=https://sepolia-rollup.arbitrum.io/rpc
```

If successful, you should see:

```bash
Finished `release` profile [optimized] target(s) in 0.47s
stripped custom section from user wasm to remove any sensitive data
contract size: 20.5 KiB (20985 bytes)
wasm size: 77.1 KiB (78903 bytes)
File used for deployment hash: ./Cargo.lock
File used for deployment hash: ./Cargo.toml
File used for deployment hash: ./examples/erc20.rs
File used for deployment hash: ./rust-toolchain.toml
File used for deployment hash: ./src/lib.rs
File used for deployment hash: ./src/main.rs
project metadata hash computed on deployment: "ffcc9fa3f5cea8e6782a9a97df2bda51f35eae24567ecbf4a7f70828b054bc69"
stripped custom section from user wasm to remove any sensitive data
contract size: 20.5 KiB (20985 bytes)
wasm data fee: 0.000118 ETH (originally 0.000099 ETH with 20% bump)
```

At first, build the Stylus contract:

```bash
cargo build --release --target wasm32-unknown-unknown \
  -Z build-std=std,panic_abort \
  -Z build-std-features=panic_immediate_abort
```

Here's how to deploy:

```bash
cargo stylus deploy \
  -e=$RPC_URL \
  --private-key=$PRIV_KEY \
  --wasm-file=$WASM_FILE \
  --no-verify \
  --deployer-address=$DEPLOYER_ADDRESS \
  --constructor-signature 'constructor(string,string,address)' \
  --constructor-args 'Stylus ERC-20 Workshop' 'MTK' '0x3f1eae7d46d88f08fc2f8ed27fcb2ab183eb2d0e'
```

The CLI will send 2 transactions to deploy and activate your program onchain.

```bash
stripped custom section from user wasm to remove any sensitive data
contract size: 20.5 KiB (20985 bytes)
wasm data fee: 0.000118 ETH (originally 0.000099 ETH with 20% bump)
deployed code at address: 0xab8e440727a38bbb180f7032ca4a8009e7b52b80
deployment tx hash: 0x721dc5c9c177448a4a929e56f0a0b161ef8ccb666646cb7b0b6e67ed39266f0c

NOTE: We recommend running cargo stylus cache bid ab8e440727a38bbb180f7032ca4a8009e7b52b80 0 to cache your activated contract in ArbOS.
Cached contracts benefit from cheaper calls. To read more about the Stylus contract cache, see
https://docs.arbitrum.io/stylus/how-tos/caching-contracts
```

You can also add your contract to the [Cache Manager](https://docs.arbitrum.io/stylus/how-tos/caching-contracts#cachemanager-contract):

```bash
cargo stylus cache bid $STYLUS_CONTRACT_ADDRESS 0 --private-key=$PRIV_KEY
```

If successful, you should see:

```bash
Checking if contract can be cached...
Sending cache bid tx...
Successfully cached contract at address: 0xab8e440727A38bBB180f7032ca4a8009E7b52B80
Sent Stylus cache bid tx with hash: 0x9839cdd19646a1cd7b2273634e918afc6587d13855f8ae4ec65b2586f4aeae2c
```

Once both steps are successful, you can interact with your program as you would with any Ethereum smart contract.

## Calling Your Program

This template includes an example of how to call and transact with your program in Rust using [ethers-rs](https://github.com/gakonst/ethers-rs) under the `examples/erc20.rs`. However, your programs are also Ethereum ABI equivalent if using the Stylus SDK. **They can be called and transacted with using any other Ethereum tooling.**

By using the program address from your deployment step above, and your wallet, you can attempt to call the counter program and increase its value in storage:

```rs
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
let token = Erc20Workshop::new(address, client);

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
```

Before running, set the following env vars or place them in a `.env` file (see: [.env.example](./.env.example)) in this project:

```
RPC_URL=http://localhost:8547
STYLUS_CONTRACT_ADDRESS=<the onchain address of your deployed program>
PRIV_KEY=<your priv key to transact with>
WASM_FILE=./target/wasm32-unknown-unknown/release/stylus_erc20_workshop.wasm
DEPLOYER_ADDRESS=0xcEcba2F1DC234f70Dd89F2041029807F8D03A990
```

Next, run:

```
cargo run --example erc20 --target=<YOUR_ARCHITECTURE>
```

Where you can find `YOUR_ARCHITECTURE` by running `rustc -vV | grep host`. For M1 Apple computers, for example, this is `aarch64-apple-darwin` and for most Linux x86 it is `x86_64-unknown-linux-gnu`
