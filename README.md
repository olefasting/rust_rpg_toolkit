# Rust RPG Toolkit

**PLEASE NOTE:**

**This project is discontinued, as most of it has been included in the internal engine we use for the Fish Fight project. As we near 1.0 for Fish Fight,
this summer, we will, in stead, release an engine core library (yet to be named) based on this. It follows the same principles as this project but will
be much more agnostic, in terms of game genre. It will also have support for multiple rendering backends**

This crate allows you to create tile-based, 2D action RPGs, using Rust amd JSON. It started out as a game project but was
separated into its own project, as it grew in scope. It uses JSON files for most of its game data and resources specification,
so that games can be created with very little interaction with the Rust code. This has the benefit of making the end product
very easy to modify, both for non-developers involved in the development process, and by end users. Modification can be done,
either by modifying a game's data files directly, or by creating user modules, which are supported, out-of-the-box.

Currently, as the name suggests, it is very RPG centric, but I am working on making it more flexible. As of writing this,
things like "victory conditions" have yet to be implemented, progression only being possible through mission/quest development,
at this stage, but I plan to design this in such a way that it allows the framework to be used for non-RPG genres, as well.

I also plan to add the possibility to create games in other perspectives than top-down. It is very easy to implement, for example,
a side scrolling view, as I already (more or less) have feature parity, as well as full compatability, with Tiled maps. Implementing
side scrolling physics is just a matter of adding a few physics properties -- mainly gravity -- at this point...

## Features

- Easy definition and modification of game data and resources, without having to touch source code or re-compile your project
- Mod support out-of-the-box with modules that can extend or replace a game's data and resources
- RPG mechanics, like character stats, items and an inventory system
- Conversion from Tiled maps and instantiation of actors and items from map properties
- Flexible AI behaviors
- Dialogue system
- Mission and reward system
- UI and menus that can be built and themed via JSON
- WebAssembly support with wasm-bindgen

## Current Milestones

- [x] Re-design actor behavior system
- [x] Define basic, default AI behaviors
- [X] Refactor collision detection
- [x] Polish pathfinding
- [ ] Expanded character creation, with cosmetic options
- [ ] Implement actor abilities (currently abilities are only implemented on items)
- [ ] Refactor the UI system
- [ ] Finalize the WASM build process

## Using the library

To get started, add the following to your `Cargo.toml` file:

```toml
[dependencies]
rust-rpg-toolkit = "0.1.0"
```

### Crate features

- `collision-between-actors` If this is enabled, actors will collide with other actors, not just the map.
  This is not recommended at this stage, as navigation does not take other actors into consideration.

### Example

```rust
use rust_rpg_toolkit::prelude::*;

const GAME_NAME: &str = "My Awesome Game";
const GAME_VERSION: &str = "0.1.0";

const CONFIG_PATH: &str = "config.json";

fn get_window_conf() -> WindowConf {
    let config = Config::load(CONFIG_PATH);

    WindowConf {
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
    name: GAME_NAME.to_string(),
    version: GAME_VERSION.to_string(),
    ..Default::default()
  };

  // Load game resources, apply modules and dispatch an event that opens the main menu when the game loop starts.
  // This puts a Resources struct, holding all the games assets and data files, including everything from modules,
  // into storage, so any code that requires access to such data, must be called after this.
  init(params).await?;
  
  /* Begin optional stuff */

  // This defines the builder used when loading scenes and it is the best way to inject your own Macroquad
  // scene node implementations into the scene tree and have them drawn when you want them to.
  // The DrawBuffers require a type that implements BufferedDraw, so implementation of Macroquad's Node trait is,
  // strictly speaking, not required. This is what it was meant to be used for, however.
  // If you don't define your own scene builder, the default one will be used. 
  SceneBuilder::new()
          .with_draw_buffer::<MyBufferedDrawImpl>(DrawStage::Actors)
          .make_default();
  
  // This is also where you want to define anything else that you reference in your game data, like custom ActorBehaviors,
  // custom ButtonBuilders that are referenced in your customized GUI theme(s), etc. 
  
  /* End optional stuff */

  // Handle event queue until encountering a Quit event
  while handle_queued_events().await? {
    // Begin frame
    begin_frame();

    // ...

    // End frame
    end_frame().await;
  }

  Ok(())
}

```

Any game you create should also have an assets folder. Copy the one included in the example project as a starting point...

The example project can be built and run using the following cargo command:

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
- [M4A1 Single sound by Kibblesbob](https://soundbible.com/1804-M4A1-Single.html) ([Creative Commons Attribution 3.0](https://creativecommons.org/licenses/by/3.0/))
- [9mm Glock 17 sound by JKirsch](https://soundbible.com/1382-9mm-Glock-17.html) ([Creative Commons Attribution 3.0](https://creativecommons.org/licenses/by/3.0/))
- Other sounds from [SciFi Weapons Pro](https://sidearm-studios.itch.io/sci-fi-weapon-sounds-pro), which is a paid asset pack. This means that you will not be able to redistribute these.

\
\
License: MIT (excluding assets from external sources)

Copyright 2021 Ole A. Sjo Fasting and [Magus](http://magus.no)
