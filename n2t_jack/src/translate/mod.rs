mod keyword;

use n2t_asm::parse::Item;

fn translate(program: &str) -> impl Iterator<Item = Result<Item, ()>> + '_ {
    program
        .lines()
        .filter_map(|line| {
            line.split_once("//")
                .map(|(line, _)| line.trim())
                .and_then(|line| (!line.is_empty()).then(|| line))
        })
        .flat_map(|instr| translate_instruction(instr));

    std::iter::empty()
}

fn translate_instruction(instruction: &str) -> impl Iterator<Item = Result<Item, ()>> {
    std::iter::empty()
}
