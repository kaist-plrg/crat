#![feature(rustc_private)]

use std::{
    fs,
    fs::File,
    path::{Path, PathBuf},
};

use clap::{Parser, ValueEnum};
use compile_util::run_compiler_on_path;
use crat::*;
use serde::Deserialize;

#[derive(Parser)]
#[command(version)]
struct Args {
    // Extern
    #[arg(long, num_args = 2, value_names = ["FROM", "TO"], help = "Resolve hint for extern functions (example: `from::foo to::foo`)")]
    resolve_function: Vec<String>,
    #[arg(long, num_args = 2, value_names = ["FROM", "TO"], help = "Resolve hint for extern static variables (example: `from::foo to::foo`)")]
    resolve_static: Vec<String>,
    #[arg(long, num_args = 2, value_names = ["FROM", "TO"], help = "Resolve hint for extern types (example: `from::foo to::foo`)")]
    resolve_type: Vec<String>,

    // Bin
    #[arg(
        long,
        value_delimiter = ',',
        help = "Main function to ignore when adding bin files"
    )]
    bin_ignore: Vec<String>,

    // Union
    #[arg(long, value_delimiter = ',', help = "Target unions to replace")]
    target_union: Vec<String>,
    #[arg(long, help = "File containing the result of points-to analysis")]
    points_to_file: Option<PathBuf>,

    #[arg(short, long, help = "Enable verbose output")]
    verbose: bool,
    #[arg(long, value_delimiter = ',', help = "Passes to run")]
    pass: Vec<Pass>,
    #[arg(long, help = "Path to the configuration file")]
    config: Option<PathBuf>,

    #[arg(long, help = "Enable in-place transformation of the input directory")]
    inplace: bool,
    #[arg(short, long, help = "Path to the output directory")]
    output: Option<PathBuf>,
    #[arg(short, long, help = "Path to the log file")]
    log_file: Option<PathBuf>,
    #[arg(help = "Path to the input directory containing c2rust-lib.rs")]
    input: PathBuf,
}

#[derive(Clone, Debug, ValueEnum, Deserialize)]
#[clap(rename_all = "lower")]
#[serde(rename_all = "lowercase")]
enum Pass {
    Extern,
    Unsafe,
    Preprocess,
    Bin,
    Check,
    OutParam,
    Lock,
    Union,
    Io,
    Pointer,
}

#[derive(Debug, Default, Deserialize)]
struct Config {
    #[serde(default)]
    r#extern: extern_resolver::Config,
    #[serde(default)]
    bin: bin_file_adder::Config,
    #[serde(default)]
    r#union: union_replacer::tag_analysis::Config,

    #[serde(default)]
    verbose: bool,
    #[serde(default)]
    passes: Vec<Pass>,
    #[serde(default)]
    inplace: bool,
    log_file: Option<PathBuf>,
    output: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let mut config = args
        .config
        .map(|path| {
            let content = fs::read_to_string(path).unwrap();
            toml::from_str::<Config>(&content).unwrap()
        })
        .unwrap_or_default();
    config.verbose |= args.verbose;
    config.passes.extend(args.pass);
    config.inplace |= args.inplace;

    if let Some(log) = args.log_file.or(config.log_file) {
        let log_file = File::create(log).unwrap();
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .without_time()
            .with_ansi(false)
            .with_level(false)
            .with_writer(log_file)
            .init();
    }

    for args in args.resolve_function.chunks(2) {
        let [from, to] = args else { panic!() };
        config
            .r#extern
            .function_hints
            .push(extern_resolver::LinkHint::new(from.clone(), to.clone()));
    }
    for args in args.resolve_static.chunks(2) {
        let [from, to] = args else { panic!() };
        config
            .r#extern
            .static_hints
            .push(extern_resolver::LinkHint::new(from.clone(), to.clone()));
    }
    for args in args.resolve_type.chunks(2) {
        let [from, to] = args else { panic!() };
        config
            .r#extern
            .type_hints
            .push(extern_resolver::LinkHint::new(from.clone(), to.clone()));
    }

    for arg in args.bin_ignore {
        config.bin.ignores.push(arg);
    }

    for u in args.target_union {
        config.r#union.target_unions.insert(u);
    }
    if let Some(file) = args.points_to_file {
        config.r#union.points_to_file.get_or_insert(file);
    }

    let dir = if let Some(mut output) = config.output.or(args.output) {
        output.push(args.input.file_name().unwrap());
        if output.exists() {
            if !output.is_dir() {
                eprintln!("{output:?} is not a directory");
                std::process::exit(1);
            }
            clear_dir(&output);
        } else if fs::create_dir_all(&output).is_err() {
            eprintln!("Cannot create {output:?}");
            std::process::exit(1);
        }
        copy_dir(&args.input, &output, true);
        output
    } else if config.inplace {
        args.input
    } else {
        eprintln!("Output directory is required unless in-place is enabled");
        std::process::exit(1)
    };
    let file = dir.join("c2rust-lib.rs");

    for pass in config.passes {
        if config.verbose {
            println!("{pass:?}");
        }
        match pass {
            Pass::Extern => {
                run_compiler_on_path(&file, |tcx| {
                    extern_resolver::resolve_extern(&config.r#extern, tcx)
                })
                .unwrap();
            }
            Pass::Unsafe => {
                run_compiler_on_path(&file, unsafe_resolver::resolve_unsafe).unwrap();
            }
            Pass::Preprocess => {
                run_compiler_on_path(&file, preprocessor::preprocess).unwrap();
            }
            Pass::Bin => {
                run_compiler_on_path(&file, |tcx| {
                    bin_file_adder::add_bin_files(&dir, &config.bin, tcx)
                })
                .unwrap();
            }
            Pass::Check => {
                run_compiler_on_path(&file, type_checker::type_check).unwrap();
            }
            Pass::OutParam => {
                todo!()
            }
            Pass::Lock => {
                todo!()
            }
            Pass::Union => {
                let _res = run_compiler_on_path(&file, |tcx| {
                    union_replacer::tag_analysis::analyze(&config.r#union, config.verbose, tcx)
                })
                .unwrap();
            }
            Pass::Io => {
                let _res =
                    run_compiler_on_path(&file, |tcx| io_replacer::replace_io(&dir, tcx)).unwrap();
            }
            Pass::Pointer => {
                todo!()
            }
        }
    }
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
