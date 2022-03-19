#![allow(dead_code)]

use structopt::*;

mod parse;
mod assemble;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "nand2tetris-assembler",
    about = "assembles assembly written for the nand2tetris vm into hardware language"
)]
struct Opt {

}

fn main() {
    println!("Hello, world!");
}
