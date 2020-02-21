use anyhow::Result;
use crate::models::File;
use domain::files::models::FileContent;

use diesel::prelude::*;

use std::env;
use dotenv::dotenv;

pub struct Repository {}

trait DbConnection {
    fn conn(&self) -> SqliteConnection;
}

impl DbConnection for Repository {
    fn conn(&self) -> SqliteConnection {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        SqliteConnection::establish(&database_url)
                         .expect(&format!("Error connecting to {}", database_url))
    }
}

impl domain::repositories::Repository for Repository {
    fn save_file(&self, file_data: FileContent) -> Result<FileContent> {
        let file_model = File { id: 0,
                                file_name: (&file_data.file_name).to_string(),
                                is_no_expires: *&file_data.is_no_expires,
                                expires: (&file_data.expires).to_string(),
                                key: (&file_data.key).to_string()};
        Ok(file_data)
    }
}
