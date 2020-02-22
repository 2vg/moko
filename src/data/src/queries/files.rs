use anyhow::Result;

use diesel::prelude::*;
use diesel::result::Error;

use crate::models::*;
use crate::schema::files;
use crate::repositories::Repository;

pub fn insert(repo: &Repository, file: NewFile) -> Result<(), Error> {
    diesel::insert_into(files::table)
           .values(&file)
           .execute(&repo.0.conn())?;
    Ok(())
}
