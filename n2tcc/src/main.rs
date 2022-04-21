extern crate core;

mod asm;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about = "n2tcc - compiler for various nand2tetris languages")]
#[clap(propagate_version = true)]
struct Opt {
    #[clap(subcommand)]
    subcommand: Language,
}

#[derive(Subcommand)]
enum Language {
    Asm(asm::Asm)
}

impl Opt {
    fn run(self) {
        self.subcommand.run();
    }
}

impl Language {
    fn run(self) {
        match self {
            Language::Asm(asm) => asm.run(),
        }
    }
}

fn main() {
    Opt::parse().run();
}
