mod arithmetic;
mod stack;

use n2t_asm::parse::Item;
use std::str::FromStr;

pub fn translate(program: &str) -> impl Iterator<Item = Result<Item, ()>> + '_ {
    program
        .lines()
        .filter_map(|line| {
            let line = line.split_once("//").map(|(x, _)| x).unwrap_or(line);
            match line.trim() {
                x if x.is_empty() => None,
                x => Some(x)
            }
        })
        .flat_map(|instr| {
            if let Ok(items) = translate_instruction(instr) {
                items.into_iter().map(Ok).collect::<Vec<_>>()
            } else {
                vec![]
            }
        })
}

fn translate_instruction(instruction: &str) -> Result<Vec<Item>, ()> {
    let mut commands = instruction.split_whitespace();
    if let Some(command) = commands.next() {
        if let Ok(op) = arithmetic::Arithmetic::from_str(command) {
            Ok(op.translate().iter().cloned().collect::<Vec<_>>())
        } else if let Ok(stack_access) = stack::Stack::from_str(command) {
            stack_access.translate(commands)
        } else {
            todo!()
        }
    } else {
        Err(())
    }
}
