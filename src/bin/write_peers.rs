extern crate database_shieldfactory;
extern crate diesel;
use std::env;

use self::database_shieldfactory::*;
use self::diesel::prelude::*;
use self::models::*;

fn main() {
    use database_shieldfactory::schema::interface::dsl::*;

    let args: Vec<String> = env::args().collect();

    let allowed_ips = &args[1];

    let endpoint = String::from("193.52.13.247");

    let public_key = &args[2];

    let connection = establish_connection();

    let mut id_interface_local: i32 = 0;
    
    let results = interface.limit(5)
        .load::<Interface>(&connection)
        .expect("Error loading interfaces");
    
    for interfaces in results {
        id_interface_local = interfaces.id_interface;
    }

    println!("id interface = {}", id_interface_local);

    let peer = create_peer(&connection, allowed_ips, &endpoint, public_key, &id_interface_local);
    println!("\nSaved peer with allowed_ips : {}, endpoint : {}, public_key : {} and id_interface : {} with id {}", allowed_ips, endpoint, public_key, id_interface_local, peer.id_peer);

}