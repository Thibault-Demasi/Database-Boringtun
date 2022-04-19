extern crate database_shieldfactory;
extern crate diesel;

use self::database_shieldfactory::*;

fn main() {
    let connection = establish_connection();

    //Je mets des trucs au hasard pour l'instant
    let email = String::from("mail.mail@mail.com");
    let verified_mail: bool = false;
    let private_salt = String::from("1234");
    let crypted_password = String::from("123456");
    let admin: bool = false;
    let public_key = String::from("");
    let interface_address = String::from("");

    let signup = create_user(&connection, &email, &verified_mail, &private_salt, &crypted_password, &admin, &public_key, &interface_address);
    println!("Saved draft {} with id {}", email, signup.id_user);

}