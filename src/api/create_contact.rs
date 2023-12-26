use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

use super::{APIAction, APISend, RPCCall, LCM_API};

#[derive(EnumString, Serialize, Debug, Clone)]
enum RecordTypeFilter {
    Contacts,
    Companies,
}

#[derive(Serialize, Debug, Clone)]
pub struct CompanyContactSearchRequest {
    // Do NOT include in API request
    #[serde(skip_serializing)]
    api_action: APIAction,

    #[serde(rename = "SearchTerms")]
    search_terms: String,

    #[serde(rename = "RecordTypeFilter")]
    record_type_filter: RecordTypeFilter,
}

impl CompanyContactSearchRequest {
    pub fn new(api_action: APIAction, search_terms: Vec<String>) -> CompanyContactSearchRequest {
        let terms = search_terms.join(", ");
        println!("terms - {:?}", terms);

        let record_type_filter = match api_action {
            APIAction::GetContacts => RecordTypeFilter::Contacts,
            APIAction::GetCompanies => RecordTypeFilter::Companies,
        };

        return CompanyContactSearchRequest {
            api_action,
            search_terms: terms,
            record_type_filter,
        };
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct CompanyContactResult {
    #[serde(rename = "ContactId")]
    pub contact_id: String,

    #[serde(rename = "AssignedTo")]
    pub assigned_to: u128,

    #[serde(rename = "IsCompany")]
    pub is_company: bool,

    #[serde(rename = "CompanyId")]
    pub company_id: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CompanyContactSearchResponse {
    #[serde(rename = "HasMoreResults")]
    pub has_more_results: bool,

    #[serde(rename = "Results")]
    pub results: Vec<CompanyContactResult>,
}

#[async_trait]
impl APISend<CompanyContactSearchResponse> for CompanyContactSearchRequest {
    async fn send(&self, api_key: &str) -> Result<CompanyContactSearchResponse> {
        let body = RPCCall::new(self.api_action.clone(), self.to_owned());
        println!("{:?}", serde_json::to_string(&body));

        let client = reqwest::Client::new();
        let res: CompanyContactSearchResponse = client
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
