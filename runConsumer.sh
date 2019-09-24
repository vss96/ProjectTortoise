#!/bin/bash
cd  $PWD/tortoise_client/src
cargo run
echo Consumer end Timestamp :  $(($(date +%s%N)/1000000))
