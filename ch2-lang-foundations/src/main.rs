use std::env;
use regex::Regex;
use crate::clap_crate::{file_cli_search, simple_cli_search};
use crate::generics::add_generics;
use crate::grep_lite::{grep, SearchTerm};
use crate::mandelbrot::{calculate_mandelbrot, render_mandelbrot};
use crate::reading_files::{read_line_functional, read_line_procedural};
use crate::regex_crate::match_regex;
use crate::rust_arrays::iterate_rust_array;
use crate::sandbox::{add_some_nums, SomeNum};

mod sandbox;
mod mandelbrot;
mod generics;
mod grep_lite;
mod rust_arrays;
mod regex_crate;
mod clap_crate;
mod reading_files;

fn main() {
    println!("Reading file procedurally:");
    read_line_procedural();

    println!("Reading file functionally:");
    read_line_functional();

    let mandelbrot = calculate_mandelbrot(
        1000,
        2.0,
        1.0,
        -1.0,
        1.0,
        100,
        24,
    );

    render_mandelbrot(mandelbrot);

    let r = add_some_nums(SomeNum::new(60), SomeNum::new(80));

    println!("{:?}", r);

    add_generics();

    let picture_term = SearchTerm::from_string(String::from("picture"));
    let term = SearchTerm::from_regex(String::from("(picture|books)"));

    iterate_rust_array();

    grep(term, 1);
    grep(picture_term, 1);

    let regex = Regex::new("(picture|books)").unwrap();

    match_regex(regex);

    let num_args = env::args().count() - 1;

    match num_args {
        0 => println!("No arguments provided"),
        1 => {
            println!("Simple CLI search:");
            simple_cli_search()
        }
        2 => {
            println!("File CLI search:");
            file_cli_search()
        },
        _ => println!("More than two arguments provided"),
    }
}
