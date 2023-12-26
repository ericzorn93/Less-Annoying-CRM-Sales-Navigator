pub mod get_contacts;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

// Constants
pub const LCM_API: &str = "https://api.lessannoyingcrm.com/v2/";

// Enums
#[derive(strum::EnumString, Serialize)]
pub enum APIAction {
    GetContacts,
    GetCompanies,
}

// Traits
#[async_trait]
pub trait APISend<R>
where
    R: DeserializeOwned,
{
    async fn send(&self, api_key: &str, api_action: APIAction) -> anyhow::Result<R>;
}

// Structs
#[derive(Serialize)]
pub struct RPCCall<P: Sized>
where
    P: Serialize,
{
    #[serde(rename = "Function")]
    function: APIAction,

    #[serde(rename = "Parameters")]
    parameters: P,
}

impl<P: Sized> RPCCall<P>
where
    P: Serialize,
{
    pub fn new(action: APIAction, body: P) -> Self {
        Self {
            function: action,
            parameters: body,
        }
    }
}
