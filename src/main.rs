extern crate dotenv;

use clap::Parser;
use lessannoyingcrm_salesnavigator::api::{create_contact, APIAction, APISend};
use lessannoyingcrm_salesnavigator::cli;
use lessannoyingcrm_salesnavigator::csv::{self, SalesNavigatorRecord};
use tokio::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse environment variables
    dotenv::dotenv()?;

    // Assert that Less Annoying CRM API Key is Defined
    let lcm_api_key = std::env::var("LCM_API_KEY").expect("LCM API Key is Needed");
    assert!(!lcm_api_key.is_empty());

    // Assert that Less Annoying CRM Personal User ID is Defined
    let lcm_personal_user_id =
        std::env::var("LCM_PERSONAL_USER_ID").expect("LCM Personal User ID is needed");

    // Establish and Parse CLI Arguments and Flags
    let args = cli::Args::parse();
    assert!(!args.file_path.is_empty());

    // Validate and Read File into Bytes
    let file_exists = fs::try_exists(&args.file_path).await?;
    if !file_exists {
        println!("File with file path {} does not exist", &args.file_path);
        std::process::exit(1);
    }

    // Aggregate Records into Chunks
    let records = csv::parser::parse_csv(&args.file_path)?;
    if records.is_empty() {
        println!("No CSV Records found");
        std::process::exit(1);
    }
    let record_chunks: Vec<Vec<SalesNavigatorRecord>> =
        records.chunks(3).map(|c| c.to_vec()).collect();

    let mid = record_chunks.len() / 2;
    let (half_one, half_two) = record_chunks.split_at(mid);
    let (half_one, half_two) = (half_one.to_vec(), half_two.to_vec());

    tokio::task::spawn(async move {
        println!("{:?}", half_one);

        let req = create_contact::CreateContactRequest::new(
            lcm_personal_user_id,
            APIAction::CreateContact,
            half_one.get(0).unwrap().get(0).unwrap(),
        );
        let res = req.send(&lcm_api_key).await.unwrap();
        println!("ContactId - {}", res.contact_id);
    });

    tokio::task::spawn(async move {
        half_two;
    });

    // Make Request

    Ok(())
}
