#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;
pub mod queries;
pub mod connection;
pub mod repositories;

use diesel::SqliteConnection;
pub type Repo = connection::Repo<SqliteConnection>;

pub use repositories::Repository;
