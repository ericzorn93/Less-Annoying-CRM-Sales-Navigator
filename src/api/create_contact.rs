use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::{APIAction, APISend, RPCCall, LCM_API};
use crate::csv::SalesNavigatorRecord;

#[derive(Serialize, Debug, Clone)]
pub struct Website {
    #[serde(rename(serialize = "Text"))]
    text: String,
}

impl Website {
    pub fn new(url: String) -> Self {
        Self { text: url }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct CreateContactRequest {
    // Do NOT include in API request
    #[serde(skip_serializing)]
    api_action: APIAction,

    #[serde(rename(serialize = "IsCompany"))]
    is_company: bool,

    #[serde(rename(serialize = "AssignedTo"))]
    assigned_to: String,

    #[serde(rename(serialize = "Name"))]
    name: String,

    #[serde(rename(serialize = "Job Title"))]
    job_title: String,

    #[serde(rename(serialize = "LinkedIn Profile"))]
    linkedin_url: String,

    #[serde(rename(serialize = "Company Name"))]
    company_name: String,

    #[serde(rename(serialize = "Division or Area Of Specialization"))]
    company_industry: String,

    #[serde(rename(serialize = "Website"))]
    websites: Vec<Website>,

    #[serde(rename(serialize = "Prospect Connections"))]
    prospect_connections: String,

    #[serde(rename(serialize = "Years In Position"))]
    years_in_position: u32,

    #[serde(rename(serialize = "Years In Company"))]
    years_in_company: u32,

    #[serde(rename(serialize = "Contact Added Date"))]
    date_contact_added: String,

    // This is going to be the name of the CSV file being
    // uploaded without the use of the file extension (.csv)
    #[serde(rename(serialize = "Linkedin List Search Query"))]
    linkedin_search_query: String,
}

impl CreateContactRequest {
    pub fn new<'a>(
        personal_user_id: String,
        api_action: APIAction,
        record: &'a SalesNavigatorRecord,
    ) -> CreateContactRequest {
        let mut websites = Vec::<Website>::new();
        websites.push(Website::new(record.company_url.clone()));

        let formatted_date = record.date_contact_added.format("%Y-%m-%d").to_string();

        return CreateContactRequest {
            api_action,
            is_company: false,
            assigned_to: personal_user_id,
            name: record.full_name(),
            job_title: record.title.to_string(),
            company_name: record.company_name.to_string(),
            linkedin_url: record.linkedin_url.to_string(),
            websites,
            company_industry: record.company_industry.to_string(),
            prospect_connections: record.prospect_connections.to_string(),
            years_in_position: record.years_in_position,
            years_in_company: record.years_in_company,
            date_contact_added: formatted_date,
            linkedin_search_query: record.file_name.to_string(),
        };
    }
}

#[async_trait]
impl APISend<CreateContactResponse> for CreateContactRequest {
    async fn send(&self, api_key: &str) -> Result<CreateContactResponse> {
        println!("Uploading {} - {}", self.name, self.company_name);

        let body = RPCCall::new(self.api_action.clone(), self.to_owned());

        let client = reqwest::Client::new();
        let res: CreateContactResponse = client
            .post(LCM_API)
            .header("Authorization", api_key)
            .json(&body)
            .send()
            .await?
            .json()
            .await?;

        Ok(res)
    }
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct CreateContactResponse {
    #[serde(rename(deserialize = "ContactId"))]
    pub contact_id: String,
}
