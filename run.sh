#!/bin/bash
cargo build

MAEBIN="./maelstrom/maelstrom"
RUSTYBIN="./target/debug/rusty-glomers"
LOGFILE="/home/rcarrier/Projects/rusty-glomers/rusty-glomers.log"
rm "$LOGFILE"

case "$1" in
    "serve")
        echo "Serving"
        sleep 1
        "$MAEBIN" serve
        ;;
    "1")
        echo "Running echo"
        sleep 5
    "$MAEBIN" test -w echo --bin "$RUSTYBIN" --node-count 1 --time-limit 10
    ;;
    
"2")
    echo "Running unique ID"
    sleep 5
    "$MAEBIN" test -w unique-ids --bin "$RUSTYBIN" --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition
    ;;
"3a")
    echo "Running (3a) Broadcast"
    sleep 5
    "$MAEBIN" test -w broadcast --bin "$RUSTYBIN" --node-count 1 --time-limit 20 --rate 10
    ;;
"3b")
    echo "Run (3b) Multi-Node broadcast"
    sleep 5
    "$MAEBIN" test -w broadcast --bin "$RUSTYBIN" --node-count 5 --time-limit 20 --rate 10
    ;;
"3c")
    echo "Run (3c) Multi-Node broadcast"
    sleep 5
    "$MAEBIN" test -w broadcast --bin "$RUSTYBIN" --node-count 5 --time-limit 20 --rate 10 --nemesis partition
    ;;
*)
    echo "not valid option, run with './run.sh number'"
    ;;
esac
