# Rust RPG Toolkit

**PLEASE NOTE:** this in early and very heavy development. API is subject to constant change, as it has newly transitioned
from being a game project to a library.

This crate allows you to create tile-based, 2D action RPGs, using Rust amd JSON. It started out as a game project but was
separated into its own project, as it grew in scope. It uses JSON files for most of its game data and resources specification,
so that games can be created with very little interaction with the Rust code. This has the benefit of making the end product
very easy to modify, both for non-developers involved in the development process, and by end users. Modification can be done,
either by modifying a game's data files directly, or by creating user modules, which are supported, out of the box.

## Features

- Easy definition and modification of game data and resources, without having to tuch source code
- Mod support out-of-the-box with modules that can extend or replace a game's data and resources
- RPG mechanics, like character stats, items and an inventory system
- Conversion from Tiled maps and instantiation of actors and items from map properties
- Flexible AI behaviors
- Dialogue system
- Mission and reward system
- WebAssembly support with wasm-bindgen

## Current Milestones

These are the tasks that currently have priority. When these are done, the goal is to push out the first
release and publish on crates.io

- [x] Re-design actor behavior system
- [x] Define basic, default AI behaviors
- [X] Refactor collision detection
- [x] Polish pathfinding
- [ ] Implement actor abilities (currently abilities are only implemented on items)
- [ ] Refactor the UI system
- [ ] Finalize the WASM build process

## Using the library

This has not been published on crates.io yet, and it will not be done until a more stable beta stage.
If you want to try it out, you might still do so, by depending on this git repository:

```toml
[dependencies]
rust-rpg-toolkit = { git = "https://github.com/olefasting/rust_rpg_toolkit.git" }
```

### Crate features

- `collision_between_actors` If this is enabled, actors will collide with other actors, not just the map.
  This is not recommended at this stage, as navigation does not take other actors into consideration.

### Example

```rust
use rust_rpg_toolkit::prelude::*;

const GAME_NAME: &'static str = "My Awesome Game";
const GAME_VERSION: &'static str = "0.1.0";

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
async fn main() -> Result<()> {
  let params = GameParams {
    game_name: GAME_NAME.to_string(),
    game_version: GAME_VERSION.to_string(),
    ..Default::default()
  };

  init_game(params).await?;

  while handle_event_queue().await? == false {
    clear_background(color::BLACK);

    update_input();
    draw_gui();

    next_frame().await;
  }

  Ok(())
}

```

Any game you create should also have an assets folder. Copy the one included in the example project as a starting point...

The example project can be built and run using the following cargo command, from the example project directory:

`cargo run --example example-project`

### CLI

The CLI crate currently just consists of a tiled map conversion tool, but it will be expanded as we go

### Further documentation

Check the [docs folder](https://github.com/olefasting/rust_rpg_toolkit/tree/master/docs) for more documentation.

## Contributing

Contributions are more than welcome. Feel free to create a PR or an issue.

## Credits

- [Wenrexa Minimal UI Kit](https://wenrexa.itch.io/kit-nesia2) (UI theme)
- [Free UI Kit #4](https://wenrexa.itch.io/ui-different02) (UI theme)
- [Neo Zero Cyberpunk City Tileset](https://yunusyanin.itch.io/neo-zero-cyberpunk-city-tileset) (map tiles and props)
- [Cyberpunk Top Down Game Asset Pack](https://rafazcruz.itch.io/cyberpunk-top-down-game-asset-pack) (currently not used but included in the repository)
- [Cyberpunk Items 16x16](https://jeresikstus.itch.io/cyberpunk-items-16x16) (currently used for all item graphics)
- [Animated Fires](https://stealthix.itch.io/animated-fires) (some animated fire effects)
- [M4A1 Single sound by Kibblesbob](https://soundbible.com/1804-M4A1-Single.html) ([Creative Commons Attribution 3.0](https://creativecommons.org/licenses/by/3.0/))
- [9mm Glock 17 sound by JKirsch](https://soundbible.com/1382-9mm-Glock-17.html) ([Creative Commons Attribution 3.0](https://creativecommons.org/licenses/by/3.0/))
- Other sounds from [SciFi Weapons Pro](https://sidearm-studios.itch.io/sci-fi-weapon-sounds-pro), which is a paid asset pack. This means that you will not be able to redistribute these.

\
\
License: MIT (excluding assets from external sources)

Copyright 2021 Ole A. Sjo Fasting and [Magus](http://magus.no)
