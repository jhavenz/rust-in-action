use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn read_line_procedural() {
    let f = File::open("readme.md").unwrap();
    let mut reader = BufReader::new(f);

    let mut line = String::new();

    loop {
        let len = reader.read_line(&mut line)
            .unwrap();
        if len == 0 {
            break
        }

        println!("{} ({} bytes long)", line, len);

        line.truncate(0);
    }
}

pub fn read_line_functional() {
    let f = File::open("readme.md").unwrap();
    let reader = BufReader::new(f);

    reader.lines()
        .for_each(|line| {
            let line = line.unwrap();
            println!("{}", line);
        });
}
