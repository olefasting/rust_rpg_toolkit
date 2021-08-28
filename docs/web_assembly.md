# WebAssembly

The library supports building to WebAssembly. Some features have not been implemented for WASM, however, awaiting a decision on how to handle persisting of user data, like save games and characters.

There is a script in the `example_project` folder named `build_wasm.sh` that you can have a look at to see how it is done.
Basically it boils down to installing the target and building:

```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown
```

To see how you can embed it into an HTML file, check out `web/index.html` in the `example_project` directory.

Currently, there are no loading screens, and it takes a while to load in a browser so please be patient.
