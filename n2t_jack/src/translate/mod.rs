mod keyword;
mod arithmetic;

use n2t_asm::parse::Item;
use std::str::FromStr;

fn translate(program: &str) -> impl Iterator<Item = Result<Item, ()>> + '_ {
    program
        .lines()
        .filter_map(|line| {
            line.split_once("//")
                .map(|(line, _)| line.trim())
                .and_then(|line| (!line.is_empty()).then(|| line))
        })
        .flat_map(|instr| translate_instruction(instr))
}

fn translate_instruction(instruction: &str) -> impl Iterator<Item = Result<Item, ()>> {
    let mut commands = instruction.split_whitespace();
    if let Some(command) = commands.next() {
        if let Ok(op) = keyword::Arithmetic::from_str(command) {
            todo!("{op:?}");
        } else if let Ok(mem_access) = keyword::Memory::from_str(command) {
            todo!("{mem_access:?}");
        }
    }

    std::iter::empty() // TODO: placeholder return value
}
