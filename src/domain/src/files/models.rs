#[derive(Clone, Debug, PartialEq)]
pub struct FileContent {
    pub file_name: String,
    pub is_no_expires: bool,
    pub expires: String,
    pub key: String
}
