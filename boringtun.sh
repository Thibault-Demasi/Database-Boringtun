#!/bin/bash


# exec curl https://sh.rustup.rs -sSf | sh &
# wait

# exec cargo install boringtun &
# wait

# sudo setcap cap_net_admin+epi ~/.cargo/bin/boringtun 

# exec cargo build --lib --no-default-features --release --target x86_64-unknown-linux-gnu &
# wait

# exec cargo build --bin boringtun --release --target x86_64-unknown-linux-gnu &
# wait

# exec cargo test &
# wait



dns='10.10.5.254'

read -rep $'Comment vous voulez appeler votre fichier de configuration (les espaces seront supprimés) ?\n' interface
if [ -z $interface ]
then
    while [ -z $interface ]
    do
        read -rep $'Vous n\'avez rien inséré.\nChoisissez un nom : ' interface
    done
fi
interface=${interface// /}
touch /var/run/wireguard/a
take_files=/var/run/wireguard/
for filenames in "$take_files"*
do    
    filenames=$(basename -- "${filenames%.*}")
    # for filenames2 in $filenames
    while [ $filenames = $interface ]
    do
        read -rep $'Le nom que vous avez choisi existe déjà.\nChoisissez en un autre : ' interface
        interface=${interface// /}
    done
done
rm /var/run/wireguard/a

allowed_ips=10.1.1.29/32
public_key=Hm7AMRnDRhYMLMBs98KwO9TeKKNof8yfP2auT+YWmH0=

exec boringtun --disable-drop-privileges 1 --log log.txt -v debug $interface &
sleep .5 &
sudo ip address add dev $interface 10.1.1.24/24
sleep .5 &
sudo wg set $interface private-key ../database_shieldfactory/privatekey peer $public_key allowed-ips $allowed_ips endpoint 193.52.13.247:51821 persistent-keepalive 25
sleep .5
sudo ip link set up dev $interface &
sleep .5
listenport=$(sudo wg | grep 'listening port' | head -1 | sed 's/[a-z]//g; s/://g; s/ //g')
# echo -e "valeur de listenport : $listenport"

cd ../database_shieldfactory 
pwd

exec cargo run --bin write_vpns -- $interface $dns $listenport &
wait

exec cargo run --bin write_peers $allowed_ips $public_key


# INSERT INTO vpnuser (email, verified_email, private_salt, crypted_password, admin, public_key, private_key, interface_address) VALUES 
# ('mail.mail@mail.mail', true, 'ABCDEF', '123456è89', false, '123publickey', '123privatekey', '10.10.1.24/24');
