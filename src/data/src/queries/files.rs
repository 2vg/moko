use anyhow::Result;

use diesel::prelude::*;
use diesel::result::Error;

use crate::schema::*;
use crate::models::files::*;
use crate::repositories::Repository;

pub fn insert(repo: &Repository, file: NewFileModel) -> Result<(), Error> {
    diesel::insert_into(files::table)
           .values(&file)
           .execute(&repo.0.conn())?;
    Ok(())
}
