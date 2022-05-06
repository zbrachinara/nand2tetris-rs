use clap::Args;
use n2t_asm::{assemble, parse};
use std::fs;
use std::io::{ErrorKind, Write};
use std::path::PathBuf;

#[derive(Args)]
pub struct Asm {
    file_name: PathBuf,
    dest_name: Option<PathBuf>,
    #[clap(short, long)]
    overwrite: bool,
    #[clap(short, long)]
    debug: bool,
}

impl Asm {
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
"The destination file already exists.\nPass in a different destination file or\
specify -o to confirm overwrite\n\n--help for more info"
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
        // parse source file (and propagate any errors)
        let (program, mut symbols) = parse::program(&file).unwrap_or_else(|e| {
            if self.debug {
                e.trace();
            }
            eprintln!("{}", e.raise());
            std::process::exit(1)
        });
        // assemble parsed code
        let code = assemble::to_string(&mut symbols, &program);

        // write to calculated destination
        dest_file
            .write_all(code.as_bytes())
            .expect("Failed to produce output for an unknown reason");
    }
}
