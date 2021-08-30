# Rust RPG Toolkit

This crate allows you to create tile-based, 2D action RPGs, using Rust amd JSON. It started out as a game project but was
separated into its own project, as it grew in scope. It uses JSON files for most of its game data and resources specification,
so that games can be created with very little interaction with the Rust code. This has the benefit of making the end product
very easy to modify, both for non-developers involved in the development process, and by end users. Modification can be done,
either by modifying a game's data files directly, or by creating user modules, which are supported, out of the box.

## Features

- Easy definition and modification of game data and resources, with JSON
- User modules, also made using JSON, that can extend or replace the game's data and resources
- RPG mechanics, like character stats, items and an inventory system
- Conversion from Tiled maps and instantiation of actors and items from map properties
- Scriptable dialogue system
- Scriptable mission and reward system
- WebAssembly support

I have decided to remove the save game feature and in stead go for a Diablo-style saving model, where characters and their progress are saved but not the maps.

There really is no need to save scene state when we can save progress both on missions and waypoints on a per-character basis, in stead.


## Contributing

Contributions are more than welcome. Feel free to create a PR or an issue.

## Example

You will need to create a macroquad main loop. You don't need to depend on macroquad, though, as the library is exposed through `rust_rpg_toolkit::prelude::*`

```rust
use rust_rpg_toolkit::prelude::*;

const GAME_NAME: &'static str = "My Awesome Game";
const GAME_VERSION: &'static str = "0.1.0";

const CONFIG_PATH: &'static str = "config.json";

fn get_window_conf() -> Conf {
    let config = Config::load(CONFIG_PATH);

    Conf {
        window_title: GAME_NAME.to_owned(),
        high_dpi: false,
        window_width: config.resolution.x as i32,
        window_height: config.resolution.y as i32,
        fullscreen: config.fullscreen,
        ..Default::default()
    }
}

#[macroquad::main(get_window_conf)]
async fn main() {
    let params = GameParams {
        game_name: GAME_NAME.to_string(),
        game_version: GAME_VERSION.to_string(),
        config_path: CONFIG_PATH.to_string(),
        ..Default::default()
    };

    run_game(params).await;
}

```

Any game you create should also have an assets folder. Copy the one included in the example project as a starting point...

The example project can be built and run using the following cargo commad:

`cargo run --example example-project`

## Further documentation

Check the [docs folder](https://github.com/olefasting/rust_rpg_toolkit/tree/master/docs) for more documentation.

## CLI

The CLI crate currently just consists of a tiled map conversion tool, but it will be expanded as we go 

## Credits

- [Wenrexa Minimal UI Kit](https://wenrexa.itch.io/kit-nesia2) (UI theme)
- [Free UI Kit #4](https://wenrexa.itch.io/ui-different02) (UI theme)
- [Neo Zero Cyberpunk City Tileset](https://yunusyanin.itch.io/neo-zero-cyberpunk-city-tileset) (map tiles and props)
- [Cyberpunk Top Down Game Asset Pack](https://rafazcruz.itch.io/cyberpunk-top-down-game-asset-pack) (currently not used but included in the repository)
- [Cyberpunk Items 16x16](https://jeresikstus.itch.io/cyberpunk-items-16x16) (currently used for all item graphics)
- [Animated Fires](https://stealthix.itch.io/animated-fires) (some animated fire effects)
- [M4A1 Single sound by Kibblesbob](https://soundbible.com/1804-M4A1-Single.html) ([Creative Commons Attribution 3.0](https://creativecommons.org/licenses/by/3.0/))
- [9mm Glock 17 sound by JKirsch](https://soundbible.com/1382-9mm-Glock-17.html) ([Creative Commons Attribution 3.0](https://creativecommons.org/licenses/by/3.0/))

\
\
License: MIT

Copyright 2021 Ole A. Sjo Fasting and [Magus](http://magus.no)
