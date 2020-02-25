use anyhow::Result;
use uuid::Uuid;

use crate::Repo;
use crate::models::*;
use crate::queries;
use domain::models::files::File;

pub struct Repository(pub Repo);

impl domain::repositories::Repository for Repository {
    fn save_file(&self, file_data: File) -> Result<File> {
        let file_model = files::NewFileModel { id: Uuid::new_v4().to_string(),
                                   file_name: &file_data.file_name,
                                   is_no_expires: file_data.is_no_expires,
                                   expires: &file_data.expires,
                                   key: &file_data.key
                                 };
        let _ = queries::files::insert(&self, file_model);
        Ok(file_data)
    }
}
