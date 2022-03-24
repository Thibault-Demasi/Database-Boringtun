extern crate database_shieldfactory;
extern crate diesel;

use self::database_shieldfactory::*;
use self::models::VpnUser;
use self::diesel::prelude::*;

fn main() {
    use database_shieldfactory::schema::vpnuser::dsl::*;

    let connection = establish_connection();
    let results = vpnuser.filter(verified_email.eq(true))
        .limit(5)
        .load::<VpnUser>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users\n", results.len());
    for users in results {
        println!("email : {}", users.email);
        println!("public key : {:?}", users.public_key);
        println!("address ip : {:?}\n", users.interface_address);
    }
}