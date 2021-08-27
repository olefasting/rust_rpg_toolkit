# Rust RPG Toolkit

This is a rename, and a repurpose, of my game codebase (Capstone).

Since it is turning into a full-fledged engine, faster than a game, I decided to spearate out the engine code and release it under the MIT license and create a new repository for my game.

For now most of your options are limited to creating a game through editing resources and data files, as well as through user mods, but more interactivity will be added as I transition this code from being a binary to a library.

To use the library, add it to your dependencies and call the `run_game` method, with a game version as a parameter (used for user mod dependency checking and more).

You will need an assets and a modules folder in your project root, as well as a config file. I recommend that you just copy these from this repo, to get started.
If you want to build for WASM, copy the bash file in the repo root to your project root, as well....

WASM build of the scenario included in this repo can be found [here](https://magus.no/static/web/index.html)

## Contributing

Contributions are more than welcome. Feel free to create a PR or an issue.

## Features

This is a work in progress but current features include (not an exhaustive list as we are adding features at a high pace):

- User modules that can extend or replace the game's data and resources
- Composable actors, from code or by JSON [assets/actors.json](https://github.com/olefasting/capstone/blob/master/assets/actors.json)
- Very basic AI behavior, with aggression levels that can be set on actor prototypes, through JSON or through code, that determine how they react to other actors of other factions. There are also visibility and noise levels, as factors determining how AI actors will react to their surroundings.
- RPG mechanics, such as character stats and various abilities
- Composable items, from code or by JSON [assets/items.json](https://github.com/olefasting/capstone/blob/master/assets/items.json), character inventory and more
- Dynamic resource loading, so textures can be added by editing [assets/resources.json](https://github.com/olefasting/capstone/blob/master/assets/resources.json) and referenced by `texture_id` in actors and items, both in-code and in the corresponding json-files
- Create maps either by writing them in JSON, manually, or by importing and converting Tiled maps
- Scriptable dialogue system (see [assets/dialogue.json](https://github.com/olefasting/capstone/blob/master/assets/dialogue.json))
- Scriptable mission and reward system (see [assets/dialogue.json](https://github.com/olefasting/capstone/blob/master/assets/missions.json))

Loading of save games and creating new games using an exported character, will be implemented in the coming days. The same goes for chapter transitions. Aside from this, entire games can be created by just editing JSON files. After depending on this repo, call `rust_rpg_toolkit::run_game(game_version)` from a macroquad main loop to get started.
A lot of RPG mechanics are also currently missing, like level up's, skills and feats. This will also be added in the very near future...

## Example

You should depend on [macroquad](https://github.com/not-fl3/macroquad), as well as my library, then a main like this.
Remember to copy dependencies from the `Cargo.toml` in this library, as it uses some patched versions of the library.

```rust
use macroquad::prelude::*;

use rust_rpg_toolkit::Config;

const GAME_VERSION: &'static str = "0.1.0";

fn window_conf() -> Conf {
    let config = Config::load();

    Conf {
        window_title: "Capstone".to_owned(),
        high_dpi: true,
        window_width: config.resolution.x as i32,
        window_height: config.resolution.y as i32,
        fullscreen: config.fullscreen,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    rust_rpg_toolkit::run_game(GAME_VERSION).await;
}
```

Any game you create should also have an assets folder. Copy the one included in this repo as a starting point...

## Further documentation

Check the [docs folder](https://github.com/olefasting/rust_rpg_toolkit/tree/master/docs) for more documentation.

\
\
\
License: MIT

Copyright 2021 Ole A. Sjo Fasting and [Magus](http://magus.no)
