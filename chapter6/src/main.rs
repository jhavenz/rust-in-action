use crate::raw_pointers::{comparing_references_and_a_box_of_t_to_several_types, creating_a_raw_pointer, identifying_a_values_address, mimicking_pointers_with_references, printing_strings_provided_by_external_sources};
use crate::the_heap::{allocating_and_deallocating_memory_on_the_heap, heap_example};
use crate::the_stack::{mutable_is_strong_password, read_only_is_strong_password};

mod raw_pointers;
mod the_stack;
mod the_heap;
mod mem_scanner;
mod program_addresses;
mod meminfo_win;

fn main() {
    println!("--- Pointers ---");
    mimicking_pointers_with_references();
    comparing_references_and_a_box_of_t_to_several_types();
    printing_strings_provided_by_external_sources();
    creating_a_raw_pointer();
    identifying_a_values_address();
    mimicking_pointers_with_references();

    let is_strong_password1 = read_only_is_strong_password("foo");
    let is_strong_password2 = read_only_is_strong_password(String::from("foobar"));
    println!("Read-only `is_strong_password` accepts: <&str -> {}> or <String -> {}>", is_strong_password1, is_strong_password2);

    let is_strong_password1 = mutable_is_strong_password("foobar");
    let is_strong_password2 = mutable_is_strong_password(String::from("foo"));
    println!("Mutable `is_strong_password` accepts: <&str -> {}> or <String -> {}>", is_strong_password1, is_strong_password2);

    let heaped_val = heap_example();
    println!("The heap example returns: {}", heaped_val);

    allocating_and_deallocating_memory_on_the_heap();

    /// Uncomment Cargo.toml lib and Use `cargo run -q -- --particles=true` to initiate the particles simulation.
    // if let Some(_) = std::env::args().find(|arg| arg == "--particles=true") {
    //     println!("Running particles:");
    //     particles::run();
    // }

    println!("--- mem_scanner ---");
    program_addresses::run();
}
