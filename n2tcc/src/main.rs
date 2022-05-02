extern crate core;

mod asm;
mod vm;

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
    Asm(asm::Asm),
    Vm(vm::Vm),
}

impl Opt {
    fn run(self) {
        match self.subcommand {
            Language::Asm(asm) => asm.run(),
            Language::Vm(vm) => vm.run(),
        }
    }
}

fn main() {
    Opt::parse().run();
}
