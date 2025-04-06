#!/bin/bash

which idf.py >/dev/null || {
    source ~/export-esp.sh >/dev/null 2>&1
}

case "$1" in
"" | "release")
    cargo build --release -F esp
    ;;
"debug")
    cargo build -F esp
    ;;
"web")
    cargo build --target wasm32-unknown-unknown --release -F wasm
    ;;
*)
    echo "Wrong argument. Only \"debug\"/\"release\" arguments are supported"
    exit 1
    ;;
esac
