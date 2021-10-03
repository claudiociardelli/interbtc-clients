mod error;

use clap::Clap;
use git_version::git_version;

//interBTC related
use error::Error;
use runtime::{
    substrate_subxt::PairSigner, 
    VaultRegistryPallet,
    InterBtcRuntime};


const VERSION: &str = git_version!(args = ["--tags"]);
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const NAME: &str = env!("CARGO_PKG_NAME");
const ABOUT: &str = env!("CARGO_PKG_DESCRIPTION");

#[derive(Clap)]
#[clap(name = NAME, version = VERSION, author = AUTHORS, about = ABOUT)]
struct Opts {
    /// Keyring / keyfile options.
    #[clap(flatten)]
    account_info: runtime::cli::ProviderUserOpts,

    /// Connection settings for the BTC Parachain.
    #[clap(flatten)]
    parachain: runtime::cli::ConnectionOpts,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, log::LevelFilter::Info.as_str()),
    );
    let opts: Opts = Opts::parse();

    let (key_pair, _) = opts.account_info.get_key_pair()?;
    let signer = PairSigner::<InterBtcRuntime, _>::new(key_pair);
    
    let parachain_config = opts.parachain;

    // let (shutdown_tx, _) = tokio::sync::broadcast::channel(16);
    
    // loop {
        let btc_parachain = parachain_config.try_connect(signer.clone()).await?;
        // tracing::info!("Get latest block hash.");
        // let something : Vec<_> = btc_parachain
        // let mut vaults = Vec::new();
        // let head = btc_parachain.get_latest_block_hash().await?;
    //     tracing::info!("Connected.");
   
    let vaults : Vec<_> = btc_parachain
        .get_all_vaults()
        .await?;

        println!("Vaults: {:#?}",vaults.clone());

    // let addr2vaults : Vec<_> = vaults
    //         .into_iter()
    //         .flat_map(|vault| {
    //             tracing::info!("Vault:{} - {:#?}",vault.id.clone(),vault.clone());
    //             vault
    //                 .wallet
    //                 .addresses
    //                 .iter()
    //                 .map(|addr| {
    //                     (*addr, vault.id.clone());
    //                     // tracing::info!("Wallet address:{:#?}",*addr);
    //                 })
    //                 .collect::<Vec<_>>()
    //         })
    //         .collect();
        // }
     Ok(())
     
    }

