use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use chrono::serde::ts_seconds;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct File {
    file_name: String,
    is_no_expires: bool,
    #[serde(with = "ts_seconds")]
    expires: DateTime<Utc>,
    key: String
}
