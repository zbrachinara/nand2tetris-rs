mod test_loader;
use bitvec::prelude::*;
use n2t_hdl::model::chip::ChipInfo;

#[test]
fn load_dmux() {
    let builder = test_loader::hdl_01();
    let ChipInfo {chip: mut dmux_4way, interface} = builder.get_chip_info("DMux4Way").unwrap();

    println!("{:?}", interface);
    assert_eq!(dmux_4way.eval(bits![1, 0, 0]), bits![1, 0, 0, 0]);
    assert_eq!(dmux_4way.eval(bits![1, 1, 0]), bits![0, 1, 0, 0]);
    assert_eq!(dmux_4way.eval(bits![1, 0, 1]), bits![0, 0, 1, 0]);
    assert_eq!(dmux_4way.eval(bits![1, 1, 1]), bits![0, 0, 0, 1]);
}
