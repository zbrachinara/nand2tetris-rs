use hardware_simulator::model::chip::build_ctx::ChipBuilder;

const DEP_ORDER: &[&'static str] = [
    "Not",
    "Not16",
    "And",
    "And16",
    "Or",
    "Or8Way",
    "Or16",
    "Xor",
    "Mux",
    "Mux16",
    "Mux4Way16",
    "Mux8Way16",
    "DMux",
    "DMux4Way",
    "DMux8Way", // up to 01: 15 elements
]
.as_slice();

pub fn hdl_01() -> ChipBuilder {
    let root = std::env::current_dir().unwrap().join("../test_files/01");

    let mut builder = ChipBuilder::new();
    DEP_ORDER
        .iter()
        .take(15)
        .for_each(|x| {
            builder.add_hdl(root.join(format!("{x}.hdl"))).unwrap();
        });

    builder
}
