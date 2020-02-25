use anyhow::Result;

pub use crate::{
    models::files::File
};

pub trait Repository {
    fn save_file(&self, file_data: File) -> Result<File>;
}
