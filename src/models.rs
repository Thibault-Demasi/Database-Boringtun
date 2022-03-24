use super::schema::interface;
use super::schema::peer;
use super::schema::vpnuser;
//use chrono::prelude::*;
// use diesel::sql_types::Timestamp;
use diesel::pg::data_types::PgTimestamp;

#[derive(Queryable)]
pub struct Interface {
    pub id_interface: i32,
    pub dns: String,
    pub listen_port: i32,
    pub interface_name: String,
    pub id_user: i32,
}

#[derive(Insertable)]
#[table_name="interface"]
pub struct NewInterface<'a>{
    pub dns: &'a str,
    pub listen_port: &'a i32,
    pub interface_name: &'a str,
    pub id_user: &'a i32,
}

#[derive(Queryable)]
pub struct Peers {
    pub id_peer: i32,
    pub allowed_ips: String,
    pub endpoint: String,
    pub public_key: String,
    pub id_interface: i32,
}

#[derive(Insertable)]
#[table_name="peer"]
pub struct NewPeer<'a>{
    pub allowed_ips: &'a str,
    pub endpoint: &'a str,
    pub public_key: &'a str,
    pub id_interface: &'a i32,
}

#[derive(Queryable)]
pub struct VpnUser {
    pub id_user: i32,
    pub email: String,
    pub verified_email: bool,
    pub private_salt: String,
    pub crypted_password: String,
    pub admin: bool,
    pub public_key: Option<String>,
    pub private_key: Option<String>,
    pub interface_address: Option<String>,
    pub created_at: PgTimestamp,
}

#[derive(Insertable)]
#[table_name="vpnuser"]
pub struct NewVpnUser<'a>{
    pub email: &'a str,
    pub verified_email: &'a bool,
    pub private_salt: &'a str,
    pub crypted_password: &'a str,
    pub admin: &'a bool,
    pub public_key: &'a str,
    pub private_key: &'a str,
    pub interface_address: &'a str,
    // pub created_at: Timestamp,
}