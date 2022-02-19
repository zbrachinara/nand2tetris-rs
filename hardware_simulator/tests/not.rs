use hardware_simulator::model::chip::build_ctx::ChipBuilder;

#[test]
fn load_step_not() {

    let hdl_dir = std::env::current_dir().unwrap().join("../test_files");
    let not_file = hdl_dir.join("Not.hdl");

    let mut builder = ChipBuilder::new();
    builder.add_hdl(not_file);
    let mut chip = builder.resolve_chip("Not").unwrap();

    assert_eq!([true].as_slice(), chip.eval(&[false]));
    assert_eq!([false].as_slice(), chip.eval(&[true]));

}