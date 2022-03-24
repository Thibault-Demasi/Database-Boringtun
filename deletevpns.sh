#!/bin/bash
take_files=/var/run/wireguard/

cd ../database_shieldfactory
echo "Voici la liste de vos interfaces"
exec cargo run --bin show_vpns &
wait

read -rep $'Saisissez le nom de l\'interface que vous voulez supprimer\n' interface
if [ -z $interface ]
then
    while [ -z $interface ]
    do
        read -rep $'Vous n\'avez rien inséré.\nChoisissez un nom : ' interface
    done
fi

interface=${interface// /}
sock=".sock"


for filenames in "$take_files"*
do    
    filenames=$(basename -- "${filenames%.*}")
    # for filenames2 in $filenames
    if [ $filenames = $interface ]
    then
        sockfiles="$interface$sock"
        rm /var/run/wireguard/$sockfiles
    fi
done

peer_to_delete=$(psql -U postgres -d vpndatabase -c "SELECT id_interface FROM interface WHERE interface_name='$interface'" | sed -n '3p' | sed 's/ //g')
echo $peer

exec cargo run --bin delete_peers $peer_to_delete &
wait
exec cargo run --bin delete_vpns -- $interface &
wait
