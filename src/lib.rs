use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;
pub mod models;
pub mod schema;
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use self::models::{Contact, NewContact};

pub fn create_contact(conn: &mut SqliteConnection, name: &str, email: &str) -> Contact {
    use crate::schema::contacts;

    let new_contact = NewContact { name, email };

    diesel::insert_into(contacts::table)
        .values(&new_contact)
        .returning(Contact::as_returning())
        .get_result(conn)
        .expect("Error saving new contact")
}
