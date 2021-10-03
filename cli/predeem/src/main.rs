mod error;

use clap::Clap;
use git_version::git_version;

//Tool code
use error::Error;

//interBTC related
use runtime::{
        H160,   
        substrate_subxt::PairSigner, 
        pallets::redeem::RequestRedeemEvent,
        RedeemPallet,
        VaultRegistryPallet,
        InterBtcRuntime,
        BtcAddress,
        AccountId
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
    connection_options: runtime::cli::ConnectionOpts,

 
    /// Settings specific to the cli tool.
    #[clap(flatten)]
    config: ToolConfig,
}

#[derive(Clap, Clone)]
pub struct ToolConfig {
    /// Amount to redeem, in satoshis
    #[clap(long, default_value = "0")]
    amount: u128,

    // /// Beneficiary Btc Wallet address. In string format
    #[clap(long, default_value = "*")]
    btc_address: String,

    /// Vault to redeem from
    #[clap(long, default_value = "")]
    vault_id: AccountId,
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
    let vault_id = config.vault_id;

    // User keys
    let (key_pair, _) = opts.account_info.get_key_pair()?;
    let signer = PairSigner::<InterBtcRuntime, _>::new(key_pair);
    
   
    // Connect to the parachain with the user keys
    let parachain_connection_options = opts.connection_options  ;
    let parachain = parachain_connection_options.try_connect(signer.clone()).await?;
    tracing::info!("Connected.");

    // Get list of premium redeem Vaults
     let result = parachain.get_premium_redeem_vaults().await;
    //  let result = parachain.get_vaults_with_issuable_tokens().await?;
     
     tracing::info!("Call done.");
     match  result {
         Ok(premium_redeem_vaults) => tracing::info!("{:#?}",premium_redeem_vaults[0]),
         Err(error) => tracing::info!("Error returned: {:?}",error),
     };

   
// Catch Error if amount below dust? check dust amount in Issue pallet? 

    // // Send redeem request
    // let redeem_id = parachain.request_redeem(amount, btc_address, &vault_id).await?;
    // tracing::info!("Redeem request sent.");

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

    // fn connect(connection_options: runtime::cli::ConnectionOpts, signer: PairSigner::<InterBtcRuntime, _>)  -> Result<InterBtcParachain, Error>{
    //     let result = connection_options.try_connect(signer.clone()).await?

    // }

