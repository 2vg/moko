#[derive(Clone, Debug, PartialEq)]
pub struct FileContent {
    file_name: String,
    is_no_expires: bool,
    expires: String,
    key: String
}

impl FileContent {
    pub fn file_name(&self) -> &str { &self.file_name }
    pub fn is_no_expires(&self) -> bool { *(&self.is_no_expires) }
    pub fn expires(&self) -> &str { &self.expires }
    pub fn key(&self) -> &str { &self.key }

    pub fn new(file_name: impl Into<String>, is_no_expires: bool, expires: impl Into<String>, key: impl Into<String>) -> FileContent {
        FileContent {
            file_name: file_name.into(),
            is_no_expires: is_no_expires,
            expires: expires.into(),
            key: key.into()
        }
    }
}
