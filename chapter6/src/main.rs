use crate::raw_pointers::{comparing_references_and_a_box_of_t_to_several_types, creating_a_raw_pointer, identifying_a_values_address, mimicking_pointers_with_references, printing_strings_provided_by_external_sources};

mod raw_pointers;


fn main() {
    println!("--- Pointers ---");
    mimicking_pointers_with_references();
    comparing_references_and_a_box_of_t_to_several_types();
    printing_strings_provided_by_external_sources();
    creating_a_raw_pointer();
    identifying_a_values_address();
    mimicking_pointers_with_references();
}
