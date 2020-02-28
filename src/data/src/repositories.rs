use anyhow::Result;

use crate::Repo;
use crate::models::*;
use crate::queries;
use domain::models::files::File;

pub struct Repository(pub Repo);

impl domain::repositories::Repository for Repository {
    fn save_file(&self, file_data: File) -> Result<File> {
        let new_file_model = files::NewFileModel::from(&file_data);
        let _query_result = queries::files::insert(&self, new_file_model);
        Ok(file_data)
    }
}
