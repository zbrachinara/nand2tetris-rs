use std::fs;
use std::fs::OpenOptions;
use std::io::{ErrorKind, Write};
use std::path::PathBuf;
use clap::Args;
use n2t_asm::{assemble, parse};

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
        // calculate appropriate file names
        let file_name = PathBuf::from(self.file_name);
        let source_name = file_name.file_stem().unwrap().to_string_lossy();
        let source_dir = file_name.parent().unwrap();
        // default destination name should be the same as source name, but .hack
        let dest_name = self.dest_name.map_or_else(
            || source_dir.join(PathBuf::from(format!("./{source_name}.hack"))),
            PathBuf::from,
        );

        // open destination file or create it if appropriate
        let mut dest_file = if self.overwrite {
            OpenOptions::new()
                .write(true)
                .create(true)
                .open(dest_name)
                .unwrap()
        } else {
            OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(dest_name)
                .unwrap_or_else(|e| match e.kind() {
                    ErrorKind::AlreadyExists => {
                        eprintln!(
                            "The destination file already exists.\n\
                        Pass in a different destination file or specify -o to confirm overwrite\n\n\
                        --help for more info"
                        );
                        std::process::exit(1)
                    }
                    _ => panic!("{e:?}"),
                })
        };

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
