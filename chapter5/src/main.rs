use crate::bit_patterns_and_types::{deconstruct_a_floating_point_number, how_u16_bit_patterns_translate_to_a_fixed_number_of_integers, inspecting_endianness, interpret_a_float_as_an_int, isolating_and_decoding_the_exponent_of_a_32bit_floating_point_number, isolating_and_decoding_the_mantissa_of_a_32bit_floating_point_number, isolating_and_decoding_the_sign_bit_of_a_32bit_floating_point_number, u16_vs_i16};
use crate::fixed_point_number_formats::mock_rand;

mod bit_patterns_and_types;
mod fixed_point_number_formats;
mod chip_8;

fn main() {
    u16_vs_i16();
    interpret_a_float_as_an_int();
    how_u16_bit_patterns_translate_to_a_fixed_number_of_integers();
    inspecting_endianness();

    println!("--- Floating point numbers ---");
    let floating_point_number: f32 = 42.42;
    let sign = isolating_and_decoding_the_sign_bit_of_a_32bit_floating_point_number(floating_point_number);
    let exponent = isolating_and_decoding_the_exponent_of_a_32bit_floating_point_number(floating_point_number);
    let mantissa = isolating_and_decoding_the_mantissa_of_a_32bit_floating_point_number(floating_point_number);

    println!("The scientific notation a 32-bit floating point number is: n = –1(sign_bit) × (mantissa) × Radix(exponent – 127(Bias))");
    println!("The resulting formula for '{:?}' is: (-1)^{sign} * {mantissa} * 2^{exponent}", floating_point_number);
    println!("Resulting to: {}", (-1.0_f32).powf(sign) * mantissa * 2.0_f32.powf(exponent));

    deconstruct_a_floating_point_number(floating_point_number);

    println!("--- Fixed point numbers ---");
    println!("max of input range: {:08b} -> {:?}", 0xff, mock_rand(0xff));
    println!("mid of input range: {:08b} -> {:?}", 0x7f, mock_rand(0x7f));
    println!("min of input range: {:08b} -> {:?}", 0x00, mock_rand(0x00));

    println!("--- CHIP-8 CPU ---");
    chip_8::adder::execute();
    chip_8::multiplier::execute();
    chip_8::caller::execute();
}
