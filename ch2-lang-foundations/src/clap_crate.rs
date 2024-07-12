use std::{env, io};
use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::{App, Arg};
use regex::Regex;

pub fn simple_cli_search() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(false))
        .get_matches();

    if !args.is_present("pattern") {
        println!("No CLI pattern provided...");
        println!();
        return;
    }

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let quote = "\
    Every face, every shop, bedroom window, public-house, and
    dark square is a picture feverishly turned--in search of what?
    It is the same with books. What do we seek through millions of pages?";

    for line in quote.lines() {
        match re.find(line) {
            Some(_) => {
                println!("CLI app matches:");

                println!("{}", line.trim());
            }
            None => (),
        }
    }
}

pub fn file_cli_search() {
    let pattern = Arg::with_name("pattern")
        .help("The pattern to search for")
        .takes_value(true)
        .required(false);

    let file_path = Arg::with_name("file path")
        .takes_value(true)
        .requires("pattern");

    let args = App::new("grep-file-lite")
        .version("0.2")
        .about("searches for patterns in files")
        .arg(pattern)
        .arg(file_path)
        .get_matches();

    if !args.is_present("pattern") {
        println!("No CLI pattern provided...");
        println!();
        return;
    }

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let file_path = args.value_of("file path").unwrap();

    let cwd = match env::current_dir() {
        Ok(cwd) => cwd.join(file_path),
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let f = File::open(cwd).unwrap();
    let reader = BufReader::new(f);

    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line.to_lowercase()) {
            Some(_) => {
                println!("CLI app matches:");
                println!("{}", line.trim());
            }
            None => (),
        }
    }
}

#[allow(dead_code)]
pub fn search_file_cli() {
    let args = App::new("grep-search-file-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("input")
            .help("File to search")
            .takes_value(true)
            .required(false))
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap_or("-");

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }
}

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}
