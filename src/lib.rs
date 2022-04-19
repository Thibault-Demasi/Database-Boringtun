pub mod schema;
pub mod models;
pub mod genip;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
// use diesel::sql_types::Timestamp;
//use chrono::prelude::*;

use self::models::{Interface, NewInterface};
use self::models::{Peers, NewPeer};
use self::models::{VpnUser, NewVpnUser};

pub fn create_interface<'a>(conn: &PgConnection, interface_name: &'a str, dns: &'a str, listen_port: &'a i32, id_user: &'a i32) -> Interface{
    use schema::interface;

    let new_interface = NewInterface{
        interface_name: interface_name,
        dns: dns,
        listen_port: listen_port,
        id_user: id_user,
    }; 

    diesel::insert_into(interface::table)
        .values(&new_interface)
        .get_result(conn)
        .expect("Error saving new interface")
}

pub fn create_peer<'a>(conn: &PgConnection, allowed_ips: &'a str, endpoint: &'a str, public_key: &'a str, id_interface: &'a i32) -> Peers{
    use schema::peer;

    let new_peer = NewPeer{
        allowed_ips: allowed_ips,
        endpoint: endpoint,
        public_key: public_key,
        id_interface: id_interface,
    };

    diesel::insert_into(peer::table)
        .values(&new_peer)
        .get_result(conn)
        .expect("Error saving new peer")
}

pub fn create_user<'a>(conn: &PgConnection, email: &'a str, verified_mail: &'a bool, private_salt: &'a str, crypted_password: &'a str, admin: &'a bool, public_key: &'a str, interface_address: &'a str) -> VpnUser{
    use schema::vpnuser;

    let new_user = NewVpnUser{
        email: email,
        verified_email: verified_mail,
        private_salt: private_salt,
        crypted_password: crypted_password,
        admin: admin,
        public_key: public_key,
        interface_address: interface_address,
        // created_at: created_at,
    }; 

    diesel::insert_into(vpnuser::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new peer")
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}