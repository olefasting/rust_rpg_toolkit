# capstone

This is an action/run 'n' gun RPG, created in Rust using [macroquad](https://github.com/not-fl3/macroquad).
The future holds huge boss fights, and a mind-nuking story line, as you progress through the world, and your own mind, to battle yourself and the archons to break free of samsara and the materialist dystopia of our not too distant future.

## License

The game is unlicensed because it has a planned commercial release, so  the game in its entirety, the IP (as it is fleshed out in the future), is mine and not to be copied.
Feel free to browse the code, however, and use anything you find for your own project(s). This means that you can copy and reuse implementation methods and bits of code, for commercial and non-commercial purposes, but you can't compile the game, with or without modifications, to distribute it.
You can, of course, compile the game and try it out, as long as it falls in under fair-use.

## Features

This is a work in progress but current features include (not an exhaustive list as we are adding features at a high pace):

- Composable actors, from code or by JSON [assets/actors.json](https://github.com/olefasting/capstone/blob/master/assets/actors.json)
- RPG mechanics, such as character stats and various abilities, currently contained in items, but a spell and feat system is in the works
- Composable items, from code of by JSON [assets/items.json](https://github.com/olefasting/capstone/blob/master/assets/items.json), character inventory and more
- Dynamic resource loading, so textures can be added by editing [assets/resources.json](https://github.com/olefasting/capstone/blob/master/assets/resources.json) and referenced by `texture_id` in actors and items, both in-code and in the corresponding json-files
- Tiled maps (to be replaced by a proprietary format, as soon as we create an in-game editor)

Copyright 2021 Ole A. Sjo Fasting

UNLICENSED
