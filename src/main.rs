#![allow(dead_code)]

use derive_more::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::{Error, Write};
use std::path::PathBuf;
use structopt::*;

mod assemble;
mod parse;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "nand2tetris-assembler",
    about = "assembles assembly written for the nand2tetris vm into hardware language",
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
        about = "Specify this flag to confirm overwriting the destination file"
    )]
    overwrite: bool,
}

fn main() {
    let opt = Opt::from_args();

    let file_name = PathBuf::from(opt.file_name);
    let source_name = file_name.file_stem().unwrap().to_string_lossy();
    let source_dir = file_name.parent().unwrap();
    // default destination name should be the same as source name, but .hack
    let dest_name = opt.dest_name.unwrap_or(
        source_dir
            .join(PathBuf::from(format!("./{source_name}.hack")))
            .to_string_lossy()
            .to_string(),
    );

    let file = fs::read_to_string(file_name).unwrap_or_else(|file_name| {
        eprintln!("File not found: {file_name:?}");
        std::process::exit(-1)
    });
    let program = parse::program(&file).unwrap_or_else(|_| {
        eprintln!("The given asm code is malformed");
        std::process::exit(-1)
    });
    let code = assemble::assemble_to_string(program);

    let mut dest_file = if opt.overwrite {
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(dest_name)
            .unwrap()
    } else {
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(dest_name)
            .unwrap()
    };

    dest_file.write_all(code.as_bytes());
}
