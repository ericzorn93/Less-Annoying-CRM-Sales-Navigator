use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SalesNavigatorRecord {
    #[serde(rename(deserialize = "First Name"))]
    pub first_name: String,

    #[serde(rename(deserialize = "Last Name"))]
    pub last_name: String,
}
