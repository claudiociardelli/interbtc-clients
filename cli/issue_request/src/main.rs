mod error;

use clap::Parser;
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
use bitcoin::PartialAddress;
use bitcoin::Network;

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
    /// Amount to issue, in satoshis
    #[clap(long, default_value = "30400")]
    amount: u128,

    /// Amount for griefing prevention, in satoshis
    #[clap(long, default_value = "500000000")]
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

    let amount = config.amount;
    let griefing_collateral = config.griefing;
    // let btc_address : BtcAddress = BtcAddress::decode_str(&config.btc_address).unwrap();
    let collateral_id  = parse_collateral_currency(&config.vault_collateral_id).unwrap();
    let wrapped_id  = parse_wrapped_currency(&config.vault_wrapped_id).unwrap();
    let vault_id = VaultId::new(config.vault_account_id, collateral_id, wrapped_id);

    // User keys
    let (key_pair, _) = opts.account_info.get_key_pair()?;
    let signer = InterBtcSigner::new(key_pair);
    let signer_account_id = signer.account_id().clone();
    tracing::info!("Signer:{:?}",signer_account_id);

    
    // Connect to the parachain with the user keys
    let parachain_config = opts.parachain;
    let parachain = parachain_config.try_connect(signer.clone()).await?;
    tracing::info!("Connected.");
    
// Catch Error if amount below dust? check dust amount in Issue pallet? 

    tracing::info!("amount:{}",amount);
    tracing::info!("griefing collateral:{}",griefing_collateral);
    // tracing::info!("vault:{:?}",vault_id);
    tracing::info!("vault account_ud:{:?}",vault_id.account_id);
    
    if opts.testmode > 0 {
        tracing::info!("Test mode, skipping request_issue call");
    } else {

        tracing::info!("Issue request starting");
        let _issue = parachain.request_issue(amount, &vault_id, griefing_collateral).await?;
        tracing::info!("Issue request completed");
    }

    // Get all active issues on the vault
    let issue_requests = parachain.get_all_active_issues().await?;
    for (issue_id, request) in issue_requests.into_iter() {
        if request.requester == signer_account_id {   
            tracing::info!("issue id:{}",issue_id);
            tracing::info!("request_status:{:?}",request.status);
            tracing::info!("request_btc:{:?}",request.btc_address);
            tracing::info!("request_btc_public_key:{:?}",request.btc_public_key);
            tracing::info!("request_requester:{}",request.requester);
            tracing::info!("request_amount:{}",request.amount);



            let issue_request = parachain.get_issue_request(issue_id).await?;
            tracing::info!("btc_address:{:?}",issue_request.btc_address);
            tracing::info!("btc_address bis:{:?}",issue_request.btc_address.encode_str(Network::Testnet));

        }
    }
 
    Ok(())
     
    }

