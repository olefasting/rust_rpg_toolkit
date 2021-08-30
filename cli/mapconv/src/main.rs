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
    let manifest: Vec<TiledMapDeclaration> = serde_json::from_str(&content)?;

    println!("Using manifest '{}':", args.manifest.to_string_lossy());

    for mut decl in manifest {
        decl.path = folder_path.join(Path::new(&decl.path)).to_string_lossy().to_string();
        decl.export_path = folder_path.join(Path::new(&decl.export_path)).to_string_lossy().to_string();

        Map::load_tiled_sync(decl.clone())?;

        println!(" '{}' => '{}'", decl.path, decl.export_path);
    }

    Ok(())
}
