use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::{APIAction, APISend, RPCCall, LCM_API};
use crate::csv::SalesNavigatorRecord;

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
}

impl CreateContactRequest {
    pub fn new<'a>(
        personal_user_id: String,
        api_action: APIAction,
        record: &'a SalesNavigatorRecord,
    ) -> CreateContactRequest {
        return CreateContactRequest {
            api_action,
            is_company: false,
            assigned_to: personal_user_id,
            name: record.full_name(),
        };
    }
}

#[async_trait]
impl APISend<CreateContactResponse> for CreateContactRequest {
    async fn send(&self, api_key: &str) -> Result<CreateContactResponse> {
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

#[derive(Deserialize, Debug, Clone)]
pub struct CreateContactResponse {
    #[serde(rename(deserialize = "ContactId"))]
    pub contact_id: String,
}
