# Capstone - Twilight of the Archons

This is an action/run 'n' gun RPG, created in Rust using [macroquad](https://github.com/not-fl3/macroquad).
The future holds huge boss fights, and a mind-nuking story line, as you progress through the world, and your own mind, to battle yourself and the archons to break free of samsara and the materialist dystopia of our not too distant future.

## Contributing

If you want to contribute, pull requests are welcome. If you want to partake in the project, we are also open to this. The prototype currently use placehoder graphics leeched from ich.io so we would love to get a good pixel artist on board. Coders are also welcome to alleviate, but we more or less have that role fulfilled already.
Send me a mail, on [simon@magus.no](mailto:simon@magus.no), if you are interested.

## License

The game is unlicensed because it has a planned commercial release, so  the game in its entirety, the IP (as it is fleshed out in the future), is ours and not to be copied.
Feel free to browse the code, however, and use anything you find for your own project(s). This means that you can copy and reuse implementation methods and bits of code, for commercial and non-commercial purposes, but you can't compile the game, with or without modifications, to distribute it.
You can, of course, compile the game and try it out, as long as it falls in under fair-use.

## Controls

Currently, controls are mapped as follows:

- `W, A, S, D` or `arrows` for movement
- `Shift` for sprint
- `Left Mouse Button` for primary ability (needs an equipped weapon)
- `Right Mouse Button` for secondary ability (needs an equipped trinket, for now)
- `R` to pick up nearby items
- `I` for inventory window
- `C` for character window

To use the primary and secondary abilities, go to Inventory and equip a weapon and a trinket....

## Features

This is a work in progress but current features include (not an exhaustive list as we are adding features at a high pace):

- Composable actors, from code or by JSON [assets/actors.json](https://github.com/olefasting/capstone/blob/master/assets/actors.json)
- RPG mechanics, such as character stats and various abilities, currently contained in items, but a spell and feat system is in the works
- Composable items, from code of by JSON [assets/items.json](https://github.com/olefasting/capstone/blob/master/assets/items.json), character inventory and more
- Dynamic resource loading, so textures can be added by editing [assets/resources.json](https://github.com/olefasting/capstone/blob/master/assets/resources.json) and referenced by `texture_id` in actors and items, both in-code and in the corresponding json-files
- Tiled maps (to be replaced by a proprietary format, as soon as we create an in-game editor)

## Credits, thanks and such...

All assets currently used are placeholders, taken from ich.io. When writing this, credits for assets are due to:

- [Neo Zero Cyberpunk City Tileset](https://yunusyanin.itch.io/neo-zero-cyberpunk-city-tileset) (map tiles and props)
- [Cyberpunk Top Down Game Asset Pack](https://rafazcruz.itch.io/cyberpunk-top-down-game-asset-pack) (currently not used but included in the repository)
- [Cyberpunk Items 16x16](https://jeresikstus.itch.io/cyberpunk-items-16x16) (currently used for all item graphics)
- [Animated Fires](https://stealthix.itch.io/animated-fires) (some animated fire effects)

### Fish Fight

I am currently a developer for [Fish Fight](https://twitter.com/fishfightgame) and my work on that project was the reason why I was introduced to [macroquad](https://github.com/not-fl3/macroquad) which, again, was the reason why I chose to reignite this project and port it to Rust.
Some boilerplate (maybe 30-40 lines of code) was yanked from Fish Fight, when I started out, and even though it is a closed project, for now, we are planning on making it open source in the near future, even though it is a commercial project. It will probably be released under the MIT license, thus being even more permissive than this project, so there should be no conflicts...

### Twilight of The Archons

The name 'Twilight of the Archons' has been inspired by the [documentary](https://www.youtube.com/watch?v=HsYTsdBCBdE) of the same name, made by Robert Bonomo of [Cactus Land Productions](http://www.thecactusland.com/). Though the documentary focus more on the worldly aspects of our current predicament and this game more on the spiritual aspects; I believe the title was such an excellent one that I sought Robert's blessing to steal it.
Blessing has been given and, even though he is not affiliated with the project in any way, I would like to give credit to him and his awesome work, here.

Copyright 2021 Ole A. Sjo Fasting and [Magus Interactive](https://magus.no)

UNLICENSED
