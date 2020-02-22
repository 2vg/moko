use crate::schema::files;

use serde::{Deserialize, Serialize};
use diesel::{Insertable, Queryable};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct File {
    pub id: String,
    pub file_name: String,
    pub is_no_expires: bool,
    pub expires: String,
    pub key: String
}

#[derive(Insertable, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[table_name = "files"]
pub struct NewFile<'a> {
    pub id: String,
    pub file_name: &'a str,
    pub is_no_expires: bool,
    pub expires: &'a str,
    pub key: &'a str
}
