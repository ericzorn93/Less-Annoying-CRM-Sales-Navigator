extern crate dotenv;

use core::time;

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
        records.chunks(15).map(|c| c.to_vec()).collect();
    let record_chunks = record_chunks[..2].to_vec();

    let mid = record_chunks.len() / 2;
    let (half_one, half_two) = record_chunks.split_at(mid);
    let (half_one, half_two) = (half_one.to_vec(), half_two.to_vec());

    // Send Requests
    let user_id_one = lcm_personal_user_id.clone();
    let api_key_one = lcm_api_key.clone();
    let task_one = tokio::task::spawn(async move {
        println!("Starting Half One");
        handle_chunks(user_id_one, api_key_one, half_one).await;
    });

    let user_id_two = lcm_personal_user_id.clone();
    let api_key_two = lcm_api_key.clone();
    let task_two = tokio::task::spawn(async move {
        println!("Starting Half Two");
        handle_chunks(user_id_two, api_key_two, half_two).await;
    });

    // Make Request
    let _ = tokio::join!(task_one, task_two);

    println!("Complete!!!");
    Ok(())
}

async fn handle_chunks(
    lcm_personal_user_id: String,
    lcm_api_key: String,
    records: Vec<Vec<SalesNavigatorRecord>>,
) {
    for chunk in &records {
        for record in chunk {
            let req = create_contact::CreateContactRequest::new(
                lcm_personal_user_id.clone(),
                APIAction::CreateContact,
                record,
            );
            let _ = req
                .send(&lcm_api_key)
                .await
                .expect("LCM Create Customer Failed");
        }

        tokio::time::sleep(time::Duration::from_millis(200)).await;
    }
}
