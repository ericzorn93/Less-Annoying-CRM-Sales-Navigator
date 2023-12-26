extern crate dotenv;

use clap::Parser;
use lessannoyingcrm_salesnavigator::api;
use lessannoyingcrm_salesnavigator::api::APIAction;
use lessannoyingcrm_salesnavigator::api::APISend;
use lessannoyingcrm_salesnavigator::cli;
use lessannoyingcrm_salesnavigator::csv;
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

    // Validate and Read File into Bytes
    let file_exists = fs::try_exists(&args.file_path).await?;
    if !file_exists {
        println!("File with file path {} does not exist", &args.file_path);
        std::process::exit(1);
    }
    let records = csv::parser::parse_csv(&args.file_path)?;
    println!("{:?}", records);

    let search_terms = vec!["Jon".to_string()];
    let req = api::get_contacts::CompanyContactSearchRequest::new(search_terms);
    let res = req.send(&lcm_api_key, APIAction::GetContacts).await?;
    println!("Response is {:?}", res);

    Ok(())
}
