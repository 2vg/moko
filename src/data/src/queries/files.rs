use anyhow::Result;

use crate::schema::files;

use diesel::prelude::*;
use diesel::result::Error;
use diesel::sql_query;

use crate::models::*;
use crate::repositories::Repository;
use domain::files::models::FileContent;

pub fn insert(repo: &Repository, file: File) -> Result<File> {
    diesel::insert_into(files::table)
           .values(&file)
           .get_result(&repo.conn())
}
