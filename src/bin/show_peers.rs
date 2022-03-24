extern crate database_shieldfactory;
extern crate diesel;

use self::database_shieldfactory::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use database_shieldfactory::schema::peer::dsl::*;

    let connection = establish_connection();
    let results = peer.limit(5)
        .load::<Peers>(&connection)
        .expect("Error loading peers");

    println!("Displaying {} peers", results.len());
    for peers in results {
        println!("{}", peers.allowed_ips);
        println!("----------\n");
        println!("{}", peers.endpoint);
        println!("----------\n");
        println!("{}", peers.public_key);
    }
}