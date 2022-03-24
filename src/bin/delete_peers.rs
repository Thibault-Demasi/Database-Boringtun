extern crate database_shieldfactory;
extern crate diesel;

use self::diesel::prelude::*;
use self::database_shieldfactory::*;
use std::env;
use self::models::Peers;

fn main() {
    use database_shieldfactory::schema::peer::dsl::*;

    let args = env::args().nth(1);

    let peer_to_delete = args.expect("Type a number").parse::<i32>().ok().expect("Type a number");

    let connection = establish_connection();

    let results = peer.load::<Peers>(&connection)
        .expect("Error loading peers");
    
    for _all_peers in results {
        let _peer_deleted = diesel::delete(peer.filter(id_interface.eq(peer_to_delete)))
            .execute(&connection)
            .expect("Error loading interfaces");
    }


    
}