#[/usr/bin/env bash]

cargo build --target wasm32-unknown-unknown
rm web/capstone.*
rm web/assets
cp -r assets web/assets
cp target/wasm32-unknown-unknown/debug/capstone.* web/
