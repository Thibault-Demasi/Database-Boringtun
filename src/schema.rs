table! {
    interface (id_interface) {
        id_interface -> Int4,
        dns -> Varchar,
        listen_port -> Int4,
        interface_name -> Varchar,
        id_user -> Int4,
    }
}

table! {
    peer (id_peer) {
        id_peer -> Int4,
        allowed_ips -> Varchar,
        endpoint -> Varchar,
        public_key -> Varchar,
        id_interface -> Int4,
    }
}

table! {
    vpnuser (id_user) {
        id_user -> Int4,
        email -> Varchar,
        verified_email -> Bool,
        private_salt -> Varchar,
        crypted_password -> Varchar,
        admin -> Bool,
        public_key -> Nullable<Varchar>,
        private_key -> Nullable<Varchar>,
        interface_address -> Nullable<Varchar>,
        created_at -> Timestamptz,
    }
}

joinable!(interface -> vpnuser (id_user));
joinable!(peer -> interface (id_interface));

allow_tables_to_appear_in_same_query!(
    interface,
    peer,
    vpnuser,
);
