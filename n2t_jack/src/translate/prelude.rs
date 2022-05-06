use n2t_asm::{parse::Item, n2tasm};

pub fn instruction_prelude() -> impl Iterator<Item = Result<Item, ()>> {
    INSTRUCTION_PRELUDE.into_iter().cloned().map(|it| Ok(it))
}

const INSTRUCTION_PRELUDE: &[Item] = &n2tasm!(
    {@256}
    {D=(A)}
    {@0}
    {M=(D)} // set stack pointer to 256

    {@1015}
    {D=(A)}
    {@1}
    {M=(D)} // set local pointer to 1015
);