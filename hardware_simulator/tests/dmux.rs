mod test_loader;

#[test]
fn load_dmux() {
    let builder = test_loader::hdl_01();
    let mut dmux_4way = builder.resolve_chip("DMux4Way").unwrap();

    println!("{:?}", dmux_4way.interface());
    assert_eq!(dmux_4way.eval([true, false, false].as_slice()), [true, false, false, false].as_slice());
    assert_eq!(dmux_4way.eval([true, true, false].as_slice()), [false, true, false, false].as_slice());
    assert_eq!(dmux_4way.eval([true, false, true].as_slice()), [false, false, true, false].as_slice());
    assert_eq!(dmux_4way.eval([true, true, true].as_slice()), [false, false, false, true].as_slice());
}