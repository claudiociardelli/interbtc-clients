use bdk::{
    bitcoin::Address, bitcoin::Network, blockchain::noop_progress, blockchain::ElectrumBlockchain,
    database::MemoryDatabase, electrum_client::Client, wallet::AddressIndex, Wallet, SignOptions,
};
use std::str::FromStr;

mod error;

use clap::Clap;
use git_version::git_version;

//Tool code
use error::Error;

//interBTC related
use runtime::{
        IssuePallet,
        InterBtcSigner,
        // BtcAddress,
        Signer,
        VaultId,
        AccountId,
        // CurrencyId,
        parse_collateral_currency,
        parse_wrapped_currency,
        };

const VERSION: &str = git_version!(args = ["--tags"]);
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const NAME: &str = env!("CARGO_PKG_NAME");
const ABOUT: &str = env!("CARGO_PKG_DESCRIPTION");

#[derive(Clap)]
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

#[derive(Clap, Clone)]
pub struct ToolConfig {
    /// Amount to issue, in satoshis
    #[clap(long, default_value = "30400")]
    amount: u128,

    /// Amount for griefing prevention, in satoshis
    #[clap(long, default_value = "10000")]
    griefing: u128,

    /// Vault to issue from - account
    #[clap(long, default_value = "5e4e52659cc440fdc150ac5cf1726d808f79905115929c4febc5d6123bb63d64")]
    vault_account_id: AccountId,

    /// Vault to redeem from - collateral
    #[clap(long, default_value = "KSM")] 
    vault_collateral_id: String,

    /// Vault to redeem from
    #[clap(long, default_value = "KBTC")] 
    vault_wrapped_id: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, log::LevelFilter::Info.as_str()),
    );
    let opts: Opts = Opts::parse();
    let config = opts.config;

    if opts.testmode > 0 {
        tracing::info!("Test mode:{:?}",opts.testmode);
    }

    //TODO Use account of bdk hello world. TO be replaced with an account provided by the calling user
    let external_descriptor = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/0'/0'/0/*)";
    let internal_descriptor = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/0'/0'/1/*)";

    // Create an in memory wallet
    let wallet: Wallet<ElectrumBlockchain, MemoryDatabase> = Wallet::new(
        external_descriptor,
        Some(internal_descriptor),
        Network::Testnet,
        MemoryDatabase::new(),
        ElectrumBlockchain::from(Client::new("ssl://electrum.blockstream.info:60002").unwrap()),
    )?;

    wallet.sync(noop_progress(), None)?;

    let balance = wallet.get_balance()?;
    println!("Wallet balance in SAT: {}", balance);

    // Interlay User keys
    let (key_pair, _) = opts.account_info.get_key_pair()?;
    let signer = InterBtcSigner::new(key_pair);
    let signer_account_id = signer.account_id().clone();
    tracing::info!("Signer:{:?}",signer_account_id);


    // Connect to the interlay parachain with the user keys
    let parachain_config = opts.parachain;
    let parachain = parachain_config.try_connect(signer.clone()).await?;
    tracing::info!("Connected.");
    
   
    // Get all pending issues on the vault
    tracing::info!("Starting scan of all pending issues for {}",signer_account_id);
    let issue_requests = parachain.get_all_active_issues().await?;
    for (issue_id, request) in issue_requests.into_iter() {
        if request.requester == signer_account_id {   
            tracing::info!("Found issue id:{}",issue_id);
            tracing::info!("   request_status:{:?}",request.status);
            tracing::info!("   request_btc:{:?}",request.btc_address);
            tracing::info!("   request_btc_public_key:{:?}",request.btc_public_key);
            tracing::info!("   request_requester:{}",request.requester);
            tracing::info!("   request_amount:{}",request.amount);

            let issue_request = parachain.get_issue_request(issue_id).await?;
            tracing::info!("   btc_address:{:?}",issue_request.btc_address);
            // tracing::info!("   btc_address bis:{:?}",issue_request.btc_address.encode_str(Network::Testnet));
            let btc_address = Address::from_str(issue_request.btc_address.to_str())?; 

            tracing::info!("Prepare btc transaction");
            let mut tx_builder = wallet.build_tx();
            tx_builder
            .add_recipient(btc_address.script_pubkey(), request.amount)
            .enable_rbf();
            let (mut psbt, tx_details) = tx_builder.finish()?;
            println!("  Transaction details: {:#?}", tx_details);
            if opts.testmode == 0 {
            let finalized = wallet.sign(&mut psbt, SignOptions::default())?;
            assert!(finalized, "Tx has not been finalized");
            } else {
                tracing::info!("Test mode, not not ifnalizing transaction");
            }

        }
    }
 
    Ok(())
     
    }

