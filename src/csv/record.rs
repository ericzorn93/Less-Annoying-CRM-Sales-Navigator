use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SalesNavigatorRecord {
    #[serde(rename(deserialize = "First Name"))]
    pub first_name: String,

    #[serde(rename(deserialize = "Last Name"))]
    pub last_name: String,

    #[serde(rename(deserialize = "Prospect Position"))]
    pub title: String,

    #[serde(rename(deserialize = "Company Name"))]
    pub company_name: String,

    #[serde(rename(deserialize = "Company Domain"))]
    pub company_url: String,

    #[serde(rename(deserialize = "Company Industry"))]
    pub company_industry: String,

    #[serde(rename(
        deserialize = "Prospect Connections",
        serialize = "prospectConnections"
    ))]
    pub prospect_connections: u128,

    #[serde(rename(deserialize = "Years in Position"))]
    pub years_in_position: u32,

    #[serde(rename(deserialize = "Years in Company"))]
    pub years_in_company: u32,

    #[serde(skip_deserializing)]
    #[serde(rename(serialize = "dateContactAdded"))]
    pub date_contact_added: chrono::DateTime<Utc>,
}

impl SalesNavigatorRecord {
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}
