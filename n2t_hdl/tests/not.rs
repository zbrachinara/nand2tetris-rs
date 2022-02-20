mod test_loader;
use bitvec::prelude::*;

#[test]
fn load_step_not() {
    let builder = test_loader::hdl_01();
    let mut chip = builder.resolve_chip("Not").unwrap();

    assert_eq!(bits![1], chip.eval(bits![0]));
    assert_eq!(bits![0], chip.eval(bits![1]));
}
