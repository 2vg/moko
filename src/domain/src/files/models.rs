use chrono::{Utc, DateTime};

#[derive(Clone, Debug, PartialEq)]
pub struct FileContent {
    file_name: String,
    is_no_expires: bool,
    expires: DateTime<Utc>,
    key: String
}
