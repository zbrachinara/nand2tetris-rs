mod arithmetic;
mod common;
mod keyword;

use n2t_asm::parse::Item;
use std::str::FromStr;

// fn translate(program: &str) -> impl Iterator<Item = Result<Item, ()>> + '_ {
pub fn translate(program: &str) -> impl Iterator<Item = Result<Item, ()>> + '_ {
    program
        .lines()
        .filter_map(|line| {
            line.split_once("//")
                .map(|(line, _)| line.trim())
                .and_then(|line| (!line.is_empty()).then(|| line))
        })
        .flat_map(|instr| if let Ok(items) = translate_instruction(instr) {
            items.into_iter().map(Ok).collect::<Vec<_>>()
        } else {
            vec![]
        })
}

fn translate_instruction(instruction: &str) -> Result<Vec<Item>, ()> {
    let mut commands = instruction.split_whitespace();
    if let Some(command) = commands.next() {
        if let Ok(op) = keyword::Arithmetic::from_str(command) {
            Ok(op.translate().iter().cloned().collect::<Vec<_>>())
        } else if let Ok(mem_access) = keyword::Memory::from_str(command) {
            todo!("{mem_access:?}")
        } else {
            todo!()
        }
    } else {
        Err(())
    }
}
