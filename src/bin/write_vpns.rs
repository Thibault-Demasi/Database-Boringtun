extern crate database_shieldfactory;
extern crate diesel;

use self::database_shieldfactory::*;
//use std::io::{stdin, Read};
use std::env;


fn main() {
    let connection = establish_connection();

    let args: Vec<String> = env::args().collect();

    let interface_name = &args[1];

    let dns = &args[2];

    let listening_port = &args[3];
    let listenport: i32 = listening_port.parse().unwrap();
    //println!("The listening port {:?} has been successfully registered in the database.", listenport);

    let id_user = 1;

    let interface = create_interface(&connection, interface_name, dns, &listenport, &id_user);
    println!("\nSaved interface name {}, dns {} and listenport {} with id {}", interface_name, dns, listenport, interface.id_interface);
}

//#[cfg(not(windows))]
//const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";