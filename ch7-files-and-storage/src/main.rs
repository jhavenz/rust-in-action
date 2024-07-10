use std::env;
use std::fs::File;
use std::path::{Path};

mod serde_basics;
mod hexdump_str;
mod hexdump_file;
mod file_ops;

fn main() {
    serde_basics::run();
    hexdump_str::run().unwrap();

    println!("--- hexdump file ---");
    let arg = env::args().last().unwrap_or_default();
    let file_path = Path::new(arg.as_str());
    let file = File::open(&file_path).map_err(|_| println!("No file given"));

    if file_path.exists() && file.is_ok() {
        println!("Dumping file: {:?}", file_path.canonicalize().unwrap());
        hexdump_file::run(&mut file.unwrap());
    }

    println!("--- file ops ---");
    // These 2 structs have the most common methods for working with files and paths
    // let path = Path::new("Cargo.toml");
    // let path_buf = PathBuf::from("Cargo.toml");


}
