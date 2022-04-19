extern crate database_shieldfactory;
extern crate diesel;

use self::diesel::prelude::*;
use self::database_shieldfactory::*;
use self::models::*;
use std::env;

use database_shieldfactory::genip;

fn main() {
    use database_shieldfactory::schema::vpnuser::dsl::*;

    let arguments: Vec<String> = env::args().collect();
    let publickey = &arguments[2];

    let mut interfaceaddress = genip::ip_generator().to_string();

    let id = env::args().nth(1).expect("publish_users requires a vpnuser id")
        .parse::<i32>().expect("invalid id");

    let connection = establish_connection();

    let results = vpnuser.filter(verified_email.eq(true))
        .load::<VpnUser>(&connection)
        .expect("Error loading users");

    for users in results {
        while Some(&interfaceaddress) == users.interface_address.as_ref() {
            println!("Existing address, search for a new ip address...");
            interfaceaddress = genip::ip_generator();
        }
    }

    let user = diesel::update(vpnuser.find(id))
        .set((verified_email.eq(true), public_key.eq(publickey), interface_address.eq(interfaceaddress)))
        .get_result::<VpnUser>(&connection)
        .expect(&format!("Unable to find user {}", id));
    
    println!("Published user {}", user.email);
}



