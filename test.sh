#!/bin/bash
umask 077
wg genkey | tee privatekey | wg pubkey > publickey


public_key=$(cat publickey)
private_key=$(cat privatekey)

#PENSER A CHANGER SINON NE MARCHERA PAS
id_user=1

exec cargo run --bin publish_users $id_user $public_key $private_key

# 's/[a-z]//g; s/-//g; s/ //g; s/_//g'