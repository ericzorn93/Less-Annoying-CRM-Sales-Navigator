use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SalesNavigatorRecord {
    #[serde(rename(deserialize = "First Name", serialize = "firstName"))]
    pub first_name: String,

    #[serde(rename(deserialize = "Last Name", serialize = "lastName"))]
    pub last_name: String,

    #[serde(rename(deserialize = "Prospect Position", serialize = "title"))]
    pub title: String,

    #[serde(rename(deserialize = "Company Name", serialize = "companyName"))]
    pub company_name: String,

    #[serde(rename(deserialize = "Company Domain", serialize = "companyUrl"))]
    pub company_url: String,

    #[serde(rename(deserialize = "Company Industry", serialize = "companyIndustry"))]
    pub company_industry: String,

    #[serde(rename(
        deserialize = "Prospect Connections",
        serialize = "prospectConnections"
    ))]
    pub prospect_connections: u128,

    #[serde(rename(deserialize = "Years in Position", serialize = "yearsInPosition"))]
    pub years_in_position: u32,

    #[serde(rename(deserialize = "Years in Company", serialize = "yearsInCompany"))]
    pub years_in_company: u32,

    #[serde(skip_deserializing)]
    #[serde(rename(serialize = "dateContactAdded"))]
    pub date_contact_added: chrono::DateTime<Utc>,
}
