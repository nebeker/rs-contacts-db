use self::models::*;
use diesel::prelude::*;
use rs_contacts_db::*;

fn main() {
    use self::schema::contacts::dsl::*;

    let new_name = "Alice";
    let new_email = "alice@example.com";

    let connection = &mut establish_connection();

    let mut created_contact = create_contact(connection, new_name, new_email);

    println!("Created new contact with ID {}", created_contact.id);

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
