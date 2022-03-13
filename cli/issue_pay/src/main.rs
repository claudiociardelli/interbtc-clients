
use std::str::FromStr;

// mod error;

use clap::Parser;
use git_version::git_version;

//Tool code
// use error::Error;
use bitcoin::PartialAddress;
//interBTC related
use runtime::{
        IssuePallet,
        InterBtcSigner,
        // BtcAddress,
        Signer,
        // VaultId,
        AccountId,
        // CurrencyId,
        // parse_collateral_currency,
        // parse_wrapped_currency,
        };
        use bdk::{
            bitcoin::Address, bitcoin::Network, blockchain::noop_progress, blockchain::ElectrumBlockchain,
            database::MemoryDatabase, electrum_client::Client, wallet::AddressIndex, Wallet, SignOptions,
        };

const VERSION: &str = git_version!(args = ["--tags"]);
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const NAME: &str = env!("CARGO_PKG_NAME");
const ABOUT: &str = env!("CARGO_PKG_DESCRIPTION");

#[derive(Parser)]
#[clap(name = NAME, version = VERSION, author = AUTHORS, about = ABOUT)]
struct Opts {
   /// Simulation mode. Transaction not sent.
   #[clap(short, long, parse(from_occurrences))]
   testmode: usize,

     /// Keyring / keyfile options containng the user's info
    #[clap(flatten)]
    account_info: runtime::cli::ProviderUserOpts,

    /// Connection settings for the BTC Parachain.
    #[clap(flatten)]
    parachain: runtime::cli::ConnectionOpts,

 
    /// Settings specific to the cli tool.
    #[clap(flatten)]
    config: ToolConfig,
}

#[derive(Parser, Clone)]
pub struct ToolConfig {
    /// Vault to issue from - account
    #[clap(long, default_value = "5e4e52659cc440fdc150ac5cf1726d808f79905115929c4febc5d6123bb63d64")]
    vault_account_id: AccountId,

    /// Vault to issue to - collateral
    #[clap(long, default_value = "KSM")] 
    vault_collateral_id: String,

    /// Vault to issue to
    #[clap(long, default_value = "KBTC")] 
    vault_wrapped_id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, log::LevelFilter::Info.as_str()),
    );
    let opts: Opts = Opts::parse();
    // let config = opts.config;

    if opts.testmode > 0 {
        tracing::info!("Test mode:{:?}",opts.testmode);
    }

    // User keys
    let (key_pair, _) = opts.account_info.get_key_pair()?;
    let signer = InterBtcSigner::new(key_pair);
    let signer_account_id = signer.account_id().clone();
    tracing::info!("Signer:{:?}",signer_account_id);


    // Connect to the parachain with the user keys
    let parachain_config = opts.parachain;
    let parachain = parachain_config.try_connect(signer.clone()).await?;
    tracing::info!("Connected.");
   
    let external_descriptor = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/0'/0'/0/*)";
    let internal_descriptor = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/0'/0'/1/*)";
  
    let wallet: Wallet<ElectrumBlockchain, MemoryDatabase> = Wallet::new(
        external_descriptor,
        Some(internal_descriptor),
        Network::Testnet,
        MemoryDatabase::new(),
        ElectrumBlockchain::from(Client::new("ssl://electrum.blockstream.info:60002").unwrap()),
    )?;

    let address = wallet.get_address(AddressIndex::New)?;
    tracing::info!("Generated Address: {}", address);

    let issue_requests = parachain.get_all_active_issues().await?;
    tracing::info!("Found {} issues", issue_requests.len());
    for (issue_id, request) in issue_requests.into_iter() {
    tracing::info!("issue id:{} - signer: {}",issue_id,request.requester);
        if request.requester == signer_account_id {   
            tracing::info!("request_status:{:?}",request.status);
            tracing::info!("request_btc:{:?}",request.btc_address);
            // tracing::info!("request_btc_public_key:{:?}",request.btc_public_key);
            tracing::info!("request_requester:{}",request.requester);
            let amount : u64 = request.amount as u64;  // no checks, I do not have that many BTC
            tracing::info!("request_amount:{}",amount);
            let issue_request = parachain.get_issue_request(issue_id).await?;
            let issue_request_btc_address_str = issue_request.btc_address.encode_str(Network::Testnet).unwrap();
            tracing::info!("btc_address desc:{:?}",issue_request.btc_address);
            tracing::info!("btc_address str:{:?}",issue_request_btc_address_str);
            let issue_request_btc_address = Address::from_str(&issue_request_btc_address_str)?; 
            tracing::info!("btc_address from str:{:?}",issue_request_btc_address_str);
            // let faucet_address = Address::from_str("mkHS9ne12qx9pS9VojpwU5xtRd4T7X7ZUt")?; //https://testnet-faucet.mempool.co/
            // tracing::info!("faucet_address from str:{:?}",faucet_address);

 
            tracing::info!("Synching wallet");
            wallet.sync(noop_progress(), None)?;
            let balance = wallet.get_balance()?;
            tracing::info!("Wallet balance in SAT: {}", balance);

            if balance < amount {
                tracing::info!("Balance too low. Cancelling payment");
            } else {


                let mut tx_builder = wallet.build_tx();
                tx_builder
                    .add_recipient(issue_request_btc_address.script_pubkey(), amount)
                    .enable_rbf();
                let (mut psbt, tx_details) = tx_builder.finish()?;
                tracing::info!("Transaction details: {:#?}", tx_details);
                // Do not sign in test mode
                if opts.testmode > 0 {
                    tracing::info!("Test mode. Not signing transaction");
                } else {
                    tracing::info!("Signing transaction");
                    let finalized = wallet.sign(&mut psbt, SignOptions::default())?;
                    assert!(finalized, "Tx has not been finalized");
                    tracing::info!("Transaction Signed: {}", finalized);
                    let raw_transaction = psbt.extract_tx();
                    let txid = wallet.broadcast(&raw_transaction)?;
                    tracing::info!(
                        "Transaction sent! TXID: {txid}.\nExplorer URL: https://blockstream.info/testnet/tx/{txid}",
                        txid = txid
                    );
                }
            }
        }
    }


    Ok(())
     
    }

