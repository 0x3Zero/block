#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

# set current working directory to script directory to run script from everywhere
cd "$(dirname "$0")"

# This script builds all subprojects and puts all created Wasm modules in one dir
cargo update --aggressive
marine build --release

mkdir -p artifacts
rm -f artifacts/*.wasm
cp target/wasm32-wasi/release/block_formatter.wasm artifacts/
marine aqua artifacts/block_formatter.wasm -s block_formatter -i Block_formatter > ./aqua/block_formatter.aqua

RUST_LOG="info" mrepl --quiet Config.toml