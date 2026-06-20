#!/usr/bin/env bash

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

cd ..
cd 1-connection && cargo build --release && cd ..
cd 2-server && cargo build --release && cd ..


cp 1-connection/target/release/connection_bin ./
cp 2-server/target/release/server_bin ./
