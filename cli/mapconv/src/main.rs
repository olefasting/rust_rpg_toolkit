use std::path::PathBuf;

use quicli::prelude::*;

use structopt::StructOpt;

use rust_rpg_toolkit::{
    json::TiledMap,
    prelude::*,
};

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "i")]
    input: PathBuf,
    #[structopt(short = "o")]
    output: PathBuf,
}

fn main() -> CliResult {
    let args = Cli::from_args();

    let json = read_file(&args.input)?;
    let tiled_map: TiledMap = serde_json::from_str(&json)?;

    Map::from(tiled_map).save(&args.output)?;

    println!("Success!");

    Ok(())
}
