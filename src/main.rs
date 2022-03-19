#![allow(dead_code)]

use std::fs;
use structopt::*;

mod assemble;
mod parse;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "nand2tetris-assembler",
    about = "assembles assembly written for the nand2tetris vm into hardware language"
)]
struct Opt {
    #[structopt(name = "FILE")]
    file_name: String,
}

fn main() {
    let opt = Opt::from_args();
    let file = fs::read_to_string(opt.file_name);


}
