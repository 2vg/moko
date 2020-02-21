use crate::schema::files;

use serde::{Deserialize, Serialize};
use diesel::{AsChangeset, Insertable, Queryable};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct File {
    pub id: i32,
    pub file_name: String,
    pub is_no_expires: bool,
    pub expires: String,
    pub key: String
}
