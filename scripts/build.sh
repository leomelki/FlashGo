#!/bin/bash

which idf.py >/dev/null || {
    source ~/export-esp.sh >/dev/null 2>&1
}

case "$1" in
"" | "release")
    rustup default esp
    cargo build --release -F esp
    ;;
"debug")
    rustup default esp
    cargo build -F esp
    ;;
"web")
    rustup default stable
    wasm-pack build --dev --no-pack --target web . -F wasm
    ;;
*)
    echo "Wrong argument. Only \"debug\"/\"release\" arguments are supported"
    exit 1
    ;;
esac
