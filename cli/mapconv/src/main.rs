use quicli::prelude::*;
use structopt::StructOpt;

use rust_rpg_toolkit::{
    map::*,
    prelude::*,
};

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "f", default_value = "assets")]
    assets_folder: String,
}

fn main() -> CliResult {
    let args = Cli::from_args();

    let manifest_path = format!("{}/tiled_maps.json", args.assets_folder);
    let content = read_file(&manifest_path)?;
    let manifest: Vec<TiledMapDeclaration> = serde_json::from_str(&content)?;

    println!("Converting maps using manifest file '{}'", manifest_path);

    for decl in manifest {
        let in_path = format!("{}/{}", args.assets_folder, decl.path);
        let out_path = format!("{}/{}", args.assets_folder, decl.export_path);
        Map::load_tiled_sync(&args.assets_folder, decl.clone())?;
        println!("Successfully converted map '{}' and exported it to '{}'", in_path, out_path);
    }

    Ok(())
}
