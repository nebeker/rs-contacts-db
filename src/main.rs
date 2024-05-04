use self::models::*;
use diesel::prelude::*;
use rs_contacts_db::*;

fn main() {
    use self::schema::contacts::dsl::*;

    let connection = &mut establish_connection();
    let results = contacts
        .filter(active.eq(true))
        .limit(5)
        .select(Contact::as_select())
        .load(connection)
        .expect("Error loading contacts");

    println!("Displaying {} contacts", results.len());
    for contact in results {
        println!("{}", contact.name);
        println!("-----------\n");
        println!("{}", contact.email);
    }
}
