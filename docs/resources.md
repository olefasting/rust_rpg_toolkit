# Resources

The file `assets/resources.json` holds definitions of all the various resources used in the game. All file paths are relative to `assets/`

## Materials

Materials can be defined in the `materials` array, and they need a unique `id`, a path to a fragment shader and a vertex shader file (both GLSL).

## Textures

Textures can be defined in the `textures` array, and they need a unique `id` and a `path` to the texture. You can also define an optional `filter_mode`, with the available options being `nearest_neighbor` and `linear`, with `nearest_neighbor` being the default, if no `filter_mode` is defined.

## Sound Effects

Sound effects can be defined in the `sound_effects` array, and they need a unique `id` and a `path` to the file.

## Music

Sound effects can be defined in the `music` array, and they need a unique `id` and a `path` to the file.
