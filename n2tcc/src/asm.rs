use clap::Args;

#[derive(Args)]
pub struct Asm {
    file_name: String,
    dest_name: Option<String>,
    #[clap(short, long)]
    overwrite: bool,
    #[clap(short, long)]
    debug: bool,
}

impl Asm {
    pub fn run(self) {

    }
}
