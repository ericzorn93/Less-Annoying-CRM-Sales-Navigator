use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::{APIAction, APISend, RPCCall, LCM_API};

#[derive(Serialize, Debug, Clone)]
pub struct CompanyContactSearchRequest {
    #[serde(rename = "SearchTerms")]
    search_terms: String,
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

impl CompanyContactSearchRequest {
    pub fn new(search_terms: Vec<String>) -> CompanyContactSearchRequest {
        let terms = search_terms.join(",");

        return CompanyContactSearchRequest {
            search_terms: terms,
        };
    }
}

#[async_trait]
impl APISend<CompanyContactSearchResponse> for CompanyContactSearchRequest {
    async fn send(
        &self,
        api_key: &str,
        api_action: APIAction,
    ) -> Result<CompanyContactSearchResponse> {
        let body = RPCCall::new(api_action, self.to_owned());

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
