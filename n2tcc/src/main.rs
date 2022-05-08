#![feature(iterator_try_collect)]

extern crate core;

mod asm;
mod common;
mod vm;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
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
