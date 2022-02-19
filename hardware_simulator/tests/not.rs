mod test_loader;
use hardware_simulator::model::chip::build_ctx::ChipBuilder;

#[test]
fn load_step_not() {

    let builder = test_loader::hdl_01();
    let mut chip = builder.resolve_chip("Not").unwrap();

    assert_eq!([true].as_slice(), chip.eval(&[false]));
    assert_eq!([false].as_slice(), chip.eval(&[true]));

}