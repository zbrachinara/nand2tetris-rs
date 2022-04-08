#![feature(iter_intersperse)]
#![feature(iterator_try_collect)]
#![warn(clippy::pedantic)]

use std::fs;
use std::fs::OpenOptions;
use std::io::{ErrorKind, Write};
use std::path::PathBuf;
use structopt::StructOpt;

mod assemble;
mod debug;
mod err;
mod parse;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "nand2tetris-assembler",
    about = "assembles assembly language written for the nand2tetris VM",
    author = "ZBrachinara (github.com/zbrachinara)",
    rename_all = "kebab"
)]
struct Opt {
    #[structopt(about = "The file to be compiled")]
    file_name: String,
    #[structopt(about = "The file to which to push the output")]
    dest_name: Option<String>,
    #[structopt(
        short,
        long,
        about = "Confirm overwriting the destination file"
    )]
    overwrite: bool,
    #[structopt(
        short,
        long,
        about = "Display backtrace and error details"
    )]
    debug: bool,
}

fn main() {
    let opt = Opt::from_args();

    let file_name = PathBuf::from(opt.file_name);
    let source_name = file_name.file_stem().unwrap().to_string_lossy();
    let source_dir = file_name.parent().unwrap();
    // default destination name should be the same as source name, but .hack
    let dest_name = opt.dest_name.map_or_else(
        || source_dir.join(PathBuf::from(format!("./{source_name}.hack"))),
        PathBuf::from,
    );

    dprintln!("Reading file...");
    let file = fs::read_to_string(file_name.clone()).unwrap_or_else(|_| {
        eprintln!("File not found: {file_name:?}");
        std::process::exit(1)
    });
    dprintln!("File read.\nParsing program...");
    let (program, mut symbols) = parse::program(&file).unwrap_or_else(|e| {
        if opt.debug {
            e.trace();
        }
        eprintln!("{}", e.raise());
        std::process::exit(1)
    });
    dprintln!("Program parsed.\nAssembling...");
    let code = assemble::to_string(&mut symbols, &program);
    dprintln!("Assembled.");

    let mut dest_file = if opt.overwrite {
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(dest_name)
            .unwrap()
    } else {
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(dest_name)
            .unwrap_or_else(|e| match e.kind() {
                ErrorKind::AlreadyExists => {
                    eprintln!(
                        "The destination file already exists.\n\
                        Pass in a different destination file or specify -o to confirm overwrite\n\n\
                        --help for more info"
                    );
                    std::process::exit(1)
                }
                _ => panic!("{e:?}"),
            })
    };

    dest_file
        .write_all(code.as_bytes())
        .expect("Failed to produce output for an unknown reason");
}
