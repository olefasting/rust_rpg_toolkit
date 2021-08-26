#[/usr/bin/env bash]

cargo build --target wasm32-unknown-unknown
rm web/capstone.*
rm -r web/assets
cp -r assets web/
rm -r web/assets/sound_effects/external
cp target/wasm32-unknown-unknown/debug/capstone.* web/
