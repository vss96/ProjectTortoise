#!/bin/bash
cd  $PWD/project_tortoise/src
echo Producer Timestamp :  $(($(date +%s%N)/1000000))
cargo run
