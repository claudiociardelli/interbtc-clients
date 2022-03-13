mod error;

use clap::Clap;
use git_version::git_version;

//Tool code
use error::Error;

//interBTC related
use runtime::{
        // RequestRedeemEvent,
        RedeemPallet,
        // InterBtcRuntime,
        InterBtcSigner,
        BtcAddress,
        VaultId,
        AccountId,
        // CurrencyId,
        parse_collateral_currency,
        parse_wrapped_currency,
        };
use bitcoin::PartialAddress;

const VERSION: &str = git_version!(args = ["--tags"]);
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const NAME: &str = env!("CARGO_PKG_NAME");
const ABOUT: &str = env!("CARGO_PKG_DESCRIPTION");

#[derive(Clap)]
#[clap(name = NAME, version = VERSION, author = AUTHORS, about = ABOUT)]
struct Opts {
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
    /// Amount to redeem, in satoshis
    #[clap(long, default_value = "20000")]
    amount: u128,

    // /// Beneficiary Btc Wallet address. In string format
    #[clap(long, default_value = "tb1qwn4juaswattvzpvmmnv5unkell304mf8nfa3w5")]
    btc_address: String,

    /// Vault to redeem from - account
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

    let amount = config.amount;
    let btc_address : BtcAddress = BtcAddress::decode_str(&config.btc_address).unwrap();
    let collateral_id  = parse_collateral_currency(&config.vault_collateral_id).unwrap();
    let wrapped_id  = parse_wrapped_currency(&config.vault_wrapped_id).unwrap();
    let vault_id = VaultId::new(config.vault_account_id, collateral_id, wrapped_id);

    // User keys
    let (key_pair, _) = opts.account_info.get_key_pair()?;
    let signer = InterBtcSigner::new(key_pair);
    
    // Connect to the parachain with the user keys
    let parachain_config = opts.parachain;
    let parachain = parachain_config.try_connect(signer.clone()).await?;
    tracing::info!("Connected.");
    
// Catch Error if amount below dust? check dust amount in Issue pallet? 

    // // Send redeem request
    let _redeem_id = parachain.request_redeem(amount, btc_address, &vault_id).await?;
    tracing::info!("Redeem request sent.");

    // Wait for redeem execution event

    // loop {
    //     tracing::info!("Waiting for RequestRedeemEvent.");
    //     parachain.on_event::<RequestRedeemEvent<InterBtcRuntime>, _, _, _>(
    //         |event| async move {
    //             tracing::info!("Received redeem request: {:?}", event);
    //             if &event.redeem_id == &redeem_id {
    //                 tracing::info!("Matching redeem_id.");
    //                 // return Ok(());
    //             }
    //             tracing::info!("redeem_id does not match.");
    //             // return Err(());
    //         },
    //         |error| tracing::error!("Error reading redeem event: {}", error.to_string()),
    //     )
    //     .await?;
    // }  
    Ok(())
     
    }

