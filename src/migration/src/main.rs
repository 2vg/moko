use barrel::backend::Sqlite;
use barrel::{types, Migration, Table};

fn main() {
    let mut m = Migration::new();
    m.create_table("files", |t: &mut Table| {
        t.add_column("id", types::text().primary(true));
        t.add_column("file_name", types::varchar(255));
        t.add_column("is_no_expires", types::boolean());
        t.add_column("expires", types::varchar(255));
        t.add_column("key", types::varchar(255));
    });

    println!("{}", m.make::<Sqlite>());
}
