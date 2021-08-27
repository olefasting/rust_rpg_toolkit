#[/usr/bin/env bash]

cargo build --target wasm32-unknown-unknown
rm web/capstone.*
cp target/wasm32-unknown-unknown/debug/capstone.* web/

rm -rf web/assets
cp -r assets web/
rm -rf web/assets/sound_effects/external

rm -rf web/modules
cp -r modules web/

rm -f web.tar.gz
tar cvzf web.tar.gz web
