#![allow(dead_code)]

use std::fs;
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
    let file = fs::read_to_string(opt.file_name).expect("File not found");

    let program = parse::program(&file);
}
