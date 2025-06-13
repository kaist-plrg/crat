#![feature(rustc_private)]

use std::path::PathBuf;

use clap::Parser;
use crat::compile_util::PassExt;

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(help = "Path to the input directory containing c2rust-lib.rs")]
    input: PathBuf,
}

fn main() {
    let args = Args::parse();
    let file = args.input.join("c2rust-lib.rs");
    crat::type_checker::TypeChecker.run_on_path(&file);
}
