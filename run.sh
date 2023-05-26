#!/bin/bash
cargo build

MAEBIN="./maelstrom/maelstrom"
RUSTYBIN="./target/debug/rusty-glomers"

case "$1" in
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
"3")
    echo "Running unique ID"
    sleep 5
    "$MAEBIN" test -w broadcast --bin "$RUSTYBIN" --node-count 1 --time-limit 20 --rate 10
    ;;
*)
    echo "not valid option, run with './run.sh number'"
    ;;
esac
