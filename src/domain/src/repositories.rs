use anyhow::Result;

pub use crate::{
    files::models::FileContent
};

pub trait Repository {
    fn save_file(&self, file_data: FileContent) -> Result<FileContent>;
}
