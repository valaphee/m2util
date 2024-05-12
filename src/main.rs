#![feature(slice_as_chunks)]

use std::path::{Path, PathBuf};

use clap::Parser;
use pack::Pack;

mod pack;
mod crypt;

#[derive(Parser)]
enum Command {
    Pack {
        input: String,
        output: String,
    },
    Unpack {
        input: String,
        output: String,
    }
}

fn main() {
    match Command::parse() {
        Command::Pack { input, output } => {
            todo!()
        }
        Command::Unpack { input, output } => {
            let input_path = Path::new(&input);
            let pack = Pack::open(input_path.with_extension("eix"), input_path.with_extension("epk"));
            for index in pack.indices() {
                let path = PathBuf::from(index.filename());
                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                std::fs::write(path, pack.get(index.id()).unwrap()).unwrap();
            }
        }
    }
}
