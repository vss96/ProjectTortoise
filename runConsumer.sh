#!/bin/bash
cd  $PWD/tortoise_client/src
cargo run 6881 200000
echo Consumer end Timestamp :  $(($(date +%s%N)/1000000))
