#[/usr/bin/env bash]
[[ -z "$1" ]] && echo "Need project name as an argument!" && exit

PROJECT_NAME="$1"

set -e

cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --out-dir target --target web "target/wasm32-unknown-unknown/release/${PROJECT_NAME}.wasm"
sed -i 's/import.*from .env.;/init.set_wasm = w => wasm = w;/;s/imports\[.env.\] =.*/return imports;/' "target/${PROJECT_NAME}.js"

cp target/example_project_bg.wasm web/
cp target/example_project.js web/

rm -rf web/snippets
cp -r target/snippets web/snippets

rm -rf web/assets
cp -r assets web/

rm -rf web/modules
cp -r modules web/

rm -f web.tar.gz
tar cvzf web.tar.gz web
