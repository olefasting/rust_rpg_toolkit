use quicli::prelude::*;

use rust_rpg_toolkit::{
    map::*,
    prelude::*,
};

const DEFAULT_TILED_MAPS_FILE_PATH: &'static str = "assets/tiled_maps.json";

#[derive(Debug, StructOpt)]
struct Cli {
    /// Input file to read
    file: String,
    /// Number of lines to read
    #[structopt(short = "n")]
    num: usize,
}

fn main() -> CliResult {
    let tiled_maps_file_path = format!("{}/tiled_maps.json", assets_path);
    let bytes = load_file(&tiled_maps_file_path).await
        .expect(&format!("Unable to find tiled maps file '{}'!", tiled_maps_file_path));
    let tiled_maps: Vec<TiledMapDeclaration> = serde_json::from_slice(&bytes)
        .expect(&format!("Unable to parse tiled maps file '{}'!", tiled_maps_file_path));
    for decl in tiled_maps {
        Map::load_tiled(&assets_path, decl.clone()).await
            .expect(&format!("Unable to convert tiled map '{}'!", decl.path));
    }
}
