#![feature(rustc_private)]

use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::Parser;
use crat::*;

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(short, long, help = "Path to the output directory")]
    output: PathBuf,
    #[arg(help = "Path to the input directory containing c2rust-lib.rs")]
    input: PathBuf,
}

fn main() {
    let mut args = Args::parse();

    args.output.push(args.input.file_name().unwrap());
    if args.output.exists() {
        assert!(args.output.is_dir());
        clear_dir(&args.output);
    } else {
        fs::create_dir(&args.output).unwrap();
    }
    copy_dir(&args.input, &args.output, true);
    let file = args.output.join("c2rust-lib.rs");

    let res = compile_util::run_compiler_on_path(&file, extern_resolver::resolve_extern).unwrap();
    res.apply();

    let res = compile_util::run_compiler_on_path(&file, unsafe_resolver::resolve_unsafe).unwrap();
    res.apply();
}

fn clear_dir(path: &Path) {
    for entry in fs::read_dir(path).unwrap() {
        let entry_path = entry.unwrap().path();
        if entry_path.is_dir() {
            let name = entry_path.file_name().unwrap();
            if name != "target" {
                fs::remove_dir_all(entry_path).unwrap();
            }
        } else {
            fs::remove_file(entry_path).unwrap();
        }
    }
}

fn copy_dir(src: &Path, dst: &Path, root: bool) {
    for entry in fs::read_dir(src).unwrap() {
        let src_path = entry.unwrap().path();
        let name = src_path.file_name().unwrap();
        let dst_path = dst.join(name);
        if src_path.is_file() {
            fs::copy(src_path, dst_path).unwrap();
        } else if src_path.is_dir() && (!root || name != "target") {
            fs::create_dir(&dst_path).unwrap();
            copy_dir(&src_path, &dst_path, false);
        }
    }
}
