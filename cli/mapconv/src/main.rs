#![feature(path_file_prefix)]

use std::path::PathBuf;

use quicli::prelude::*;
use structopt::StructOpt;

use rust_rpg_toolkit::{
    json::TiledMap,
    prelude::*,
};

#[derive(StructOpt, Debug)]
#[structopt(name = "mapconv")]
struct Cli {
    #[structopt(short, long)]
    map_id: Option<String>,
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,
    #[structopt(name = "OUT", parse(from_os_str))]
    output: PathBuf,
}

fn main() -> CliResult {
    let args = Cli::from_args();

    let map_id = match &args.map_id {
        Some(map_id) => map_id,
        _ => {
            args.output
                .file_prefix()
                .unwrap()
                .to_str()
                .unwrap()
        }
    };

    let json = read_file(args.file)?;
    let tiled_map = serde_json::from_str(&json)?;
    let map = Map::from_tiled(&map_id, tiled_map);
    map.save(args.output);

    println!("Success!");

    Ok(())
}
