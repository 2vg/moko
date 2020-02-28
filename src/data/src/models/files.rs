use crate::schema::files;
use domain::models::files::File;

use uuid::Uuid;
use serde::{Deserialize, Serialize};
use diesel::{Insertable, Queryable};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FileModel {
    pub id: String,
    pub file_name: String,
    pub is_no_expires: bool,
    pub expires: String,
    pub key: String
}

#[derive(Insertable, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[table_name = "files"]
pub struct NewFileModel<'a> {
    pub id: String,
    pub file_name: &'a str,
    pub is_no_expires: bool,
    pub expires: &'a str,
    pub key: &'a str
}

impl<'a> From<&'a File> for NewFileModel<'a> {
    fn from(f: &'a File) -> Self {
        NewFileModel {
            id: Uuid::new_v4().to_string(),
            file_name: &f.file_name,
            is_no_expires: f.is_no_expires,
            expires: &f.expires,
            key: &f.key
        }
    }
}
