#!/bin/bash

name="wg0"

psql -U postgres -d vpndatabase -c "SELECT id_interface FROM interface WHERE interface_name='$name'" | sed -n '3p' | sed 's/ //g'