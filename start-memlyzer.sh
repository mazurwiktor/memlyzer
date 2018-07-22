#/usr/bin/env bash

if [ $(id -u) -ne 0 ];
then 
    echo "You need to be root to use memlyzer"
    echo "[sudo] $0" 
    exit
fi

cd `dirname "$(readlink -f "$0")"` && ./target/release/memlyzer