use std::path::Path;

use chrono::Utc;

use super::record::SalesNavigatorRecord;

pub fn parse_csv<P: AsRef<Path>>(
    file_path: P,
    file_name: &str,
) -> Result<Vec<SalesNavigatorRecord>, csv::Error> {
    let mut reader = csv::Reader::from_path(file_path.as_ref())?;
    let mut records = Vec::<SalesNavigatorRecord>::new();

    for result in reader.deserialize::<SalesNavigatorRecord>() {
        let mut record = result?;
        record.date_contact_added = Utc::now();
        record.file_name = file_name.to_string();
        records.push(record);
    }

    Ok(records)
}
