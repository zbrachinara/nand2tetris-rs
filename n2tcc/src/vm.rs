use std::{path::PathBuf, io::ErrorKind, fs};

use clap::Args;

#[derive(Args)]
pub struct Vm {
    file_name: PathBuf,
    dest_name: Option<PathBuf>,
    #[clap(short, long)]
    overwrite: bool,
    #[clap(short, long)]
    debug: bool,
}

impl Vm {
    pub fn run(self) {
        // calculate appropriate file names
        let file_name = self.file_name;
        let source_name = file_name.file_stem().unwrap().to_string_lossy();
        let source_dir = file_name.parent().unwrap();

        // if not provided, default destination name should be the same as source name, but .hack
        let dest_name = super::common::calculate_destination(
            self.dest_name,
            || source_dir.join(PathBuf::from(source_name.to_string())),
            "hack",
        );

        // open destination file or create it if appropriate
        let mut dest_file =
            super::common::open_file(dest_name, self.overwrite).unwrap_or_else(|e| {
                match e.kind() {
                    ErrorKind::AlreadyExists => {
                        eprintln!(
                            "The destination file already exists.\n\
                        Pass in a different destination file or specify -o to confirm overwrite\n\n\
                        --help for more info"
                        );
                        std::process::exit(1)
                    }
                    _ => panic!("{e:?}"),
                }
            });

        // read source file
        let file = fs::read_to_string(file_name.clone()).unwrap_or_else(|_| {
            eprintln!("File not found: {file_name:?}");
            std::process::exit(1)
        });

        todo!()
    }
}
