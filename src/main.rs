extern crate dotenv;

use clap::Parser;
use lessannoyingcrm_salesnavigator::cli;
use tokio::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse environment variables
    dotenv::dotenv()?;

    // Assert that Less Annoying CRM API Key is Defined
    let lcm_api_key = std::env::var("LCM_API_KEY").expect("LCM API Key is Needed");
    assert!(!lcm_api_key.is_empty());

    // Establish and Parse CLI Arguments and Flags
    let args = cli::Args::parse();
    assert!(!args.file_path.is_empty());

    // Validate File
    let file_exists = fs::try_exists(&args.file_path).await?;
    if !file_exists {
        println!("File with file path {} does not exist", args.file_path);
        std::process::exit(1);
    }

    Ok(())
}
