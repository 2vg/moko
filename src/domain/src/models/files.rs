#[derive(Clone, Debug, PartialEq)]
pub struct File {
    pub file_name: String,
    pub is_no_expires: bool,
    pub expires: String,
    pub key: String
}

impl File {
    pub fn new(file_name: impl Into<String>, is_no_expires: bool, expires: impl Into<String>, key: impl Into<String>) -> File {
        File {
            file_name: file_name.into(),
            is_no_expires: is_no_expires,
            expires: expires.into(),
            key: key.into()
        }
    }
}
