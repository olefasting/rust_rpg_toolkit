use std::path::{
    Path,
    PathBuf,
};

use quicli::prelude::*;
use structopt::StructOpt;

use rust_rpg_toolkit::{
    map::*,
    prelude::*,
};

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "m", default_value = "tiled_manifest.json")]
    manifest: PathBuf,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    let folder_path = remove_filename(PathBuf::from(&args.manifest));

    let content = read_file(&args.manifest)?;
    let manifest: Vec<TiledMapDefinition> = serde_json::from_str(&content)?;

    println!("Using manifest '{}':", args.manifest.to_string_lossy());

    for mut def in manifest {
        def.path = folder_path.join(Path::new(&def.path)).to_string_lossy().to_string();
        def.export_path = folder_path.join(Path::new(&def.export_path)).to_string_lossy().to_string();

        Map::load_tiled_sync(def.clone())?;

        println!(" '{}' => '{}'", def.path, def.export_path);
    }

    Ok(())
}
