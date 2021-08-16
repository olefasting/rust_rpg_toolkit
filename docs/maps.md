# Maps

The game uses a proprietary map schema inspired by Tiled and can currently import these maps directly, or import Tiled maps and convert them, on the fly, to the internal schema. In the future, a map editor will be made and the dependency on Tiled can be dropped.

A thorough documentation of the schema and its implementation will be added in the future, so for now this rough description will have to suffice.

Maps consist of a root object with some metadata, as well as Vec's containing layers and tilesets (HashMaps are used internally for easier indexing by IDs).
Layers consist of a small amount of metadata, and a Vec of tiles, represented by tile_ids (u32) that points to a tile within a tileset. This is used, both to identify the specific tile to draw, as well as the relevant tilest, as tile ids are continuous across tilesets. Every tileset contains members named `first_tile_id` and `tile_cnt`, so by iterating the tilesets, you can figure out which one the tile references, based on the tile id. Internally, the tiles are represented by a struct and the `tile_id` member is relative to the tileset to be used, as this struct also contain a tileset id that can be used to index the relevant tileset.

There are also object layers that can be used to represent non-tile data, like spawn points, items, actors, etc.

For more information, reference the data structures in the [json module](https://github.com/olefasting/capstone/blob/master/src/json.rs), which holds the intermediate files that are used for serializing and deserializing of files for the project.
