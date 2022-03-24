extern crate database_shieldfactory;
extern crate diesel;

use self::diesel::prelude::*;
use self::database_shieldfactory::*;
use std::env;

fn main() {
    use database_shieldfactory::schema::interface::dsl::*;

    let args: Vec<String> = env::args().collect();

    let interfacto_to_delete = &args[1];

    let connection = establish_connection();

    let num_deleted = diesel::delete(interface.filter(interface_name.like(interfacto_to_delete)))
        .execute(&connection)
        .expect("Error deleting interfaces");

    println!("Deleted {} interface", num_deleted);
}