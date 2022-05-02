use clap::Args;

#[derive(Args)]
pub struct Vm {
    file_name: String,
    dest_name: Option<String>,
    #[clap(short, long)]
    overwrite: bool,
    #[clap(short, long)]
    debug: bool,
}

impl Vm {
    pub fn run(self) {}
}
