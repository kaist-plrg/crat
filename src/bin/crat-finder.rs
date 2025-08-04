#![feature(rustc_private)]

use std::path::PathBuf;

use clap::Parser;
use compile_util::run_compiler_on_path;
use crat::*;

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(help = "Path to the input directory containing c2rust-lib.rs")]
    input: PathBuf,
}

fn main() {
    let args = Args::parse();
    let file = args.input.join("c2rust-lib.rs");
    run_compiler_on_path(&file, finder::unsafe_finder::find_unsafe).unwrap();
}
