extern crate database_shieldfactory;
extern crate diesel;

use self::database_shieldfactory::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use database_shieldfactory::schema::interface::dsl::*;

    let connection = establish_connection();
    let results = interface.limit(5)
        .load::<Interface>(&connection)
        .expect("Error loading interfaces");

    println!("Displaying {} interfaces\n", results.len());
    let mut i = 1;
    for interfaces in results {
        println!("Interface number {}", i);
        println!("id : {}", interfaces.id_interface);
        println!("Interface name : {}", interfaces.interface_name);
        println!("dns : {}", interfaces.dns);
        println!("listen_port : {}\n", interfaces.listen_port);
        i += 1;
    }
}