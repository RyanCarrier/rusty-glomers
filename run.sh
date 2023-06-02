#!/bin/bash
#
EARLY="1-3c"
MAEBIN="./maelstrom/maelstrom"
RUSTYBIN="./target/release/rusty-glomers"
LOGFILE="/home/rcarrier/Projects/rusty-glomers/rusty-glomers.log"
rm "$LOGFILE"

PRE="cargo build --release --bin rusty-glomers --target-dir=./target --manifest-path="
BUILD123C="$PRE$EARLY/Cargo.toml"
BUILD="$PRE$1/Cargo.toml"
export CARGO_TARGET_DIR="./"
print_wait() {
    echo "Running ($1) $2"
    sleep 2
}

case "$1" in
"serve")
    print_wait "$1" "serve"
    "$MAEBIN" serve
    ;;
"1")
    $BUILD123C
    print_wait "$1" "Echo"
    "$MAEBIN" test -w echo --bin "$RUSTYBIN" --node-count 1 --time-limit 10
    ;;

"2")
    $BUILD123C
    print_wait "$1" "Running unique ID"
    "$MAEBIN" test -w unique-ids --bin "$RUSTYBIN" --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition
    ;;
"3a")
    $BUILD123C
    print_wait "$1" "Broadcast"
    "$MAEBIN" test -w broadcast --bin "$RUSTYBIN" --node-count 1 --time-limit 20 --rate 10
    ;;
"3b")
    $BUILD123C
    print_wait "$1" "Multi-Node broadcast"
    "$MAEBIN" test -w broadcast --bin "$RUSTYBIN" --node-count 5 --time-limit 20 --rate 10
    ;;
"3c")
    $BUILD123C
    print_wait "$1" "Multi-Node broadcast"
    "$MAEBIN" test -w broadcast --bin "$RUSTYBIN" --node-count 5 --time-limit 20 --rate 10 --nemesis partition
    ;;
"3d")
    $BUILD
    print_wait "$1" "Multi-Node broadcast"
    "$MAEBIN" test -w broadcast --bin "$RUSTYBIN" --node-count 25 --time-limit 20 --rate 100 --latency 100
    ;;
"3e")
    $BUILD
    print_wait "$1" "Multi-Node broadcast"
    "$MAEBIN" test -w broadcast --bin "$RUSTYBIN" --node-count 25 --time-limit 20 --rate 100 --latency 100
    ;;
*)
    echo "not valid option, run with './run.sh number'"
    ;;
esac
