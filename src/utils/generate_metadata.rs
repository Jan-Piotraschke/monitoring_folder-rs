use chrono::{Utc, Local};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub version: String,
    pub generated_on: String,
    pub uuid: String,
    pub description: String,
    pub author: String,
    pub standards: Vec<String>,
    pub comment: String,
    pub timezone: String,
}

pub fn generate_metadata() -> Metadata {
    let local_time = Local::now();
    let timezone = local_time.offset().to_string();

    Metadata {
        version: "1.0".to_string(),
        generated_on: Utc::now().to_rfc3339(),
        uuid: Uuid::new_v4().to_string(),
        description: "File size tracking for .log and .txt files".to_string(),
        author: "Jan Piotraschke".to_string(),
        standards: vec!["ISO 8601".to_string(), "ISO 19115".to_string()],
        comment: "This JSON file tracks the number of lines in .log and .txt files within the specified directory.".to_string(),
        timezone,
    }
}
