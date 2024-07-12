use std::ops::Sub;

const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

/// The data type determines what a sequence of bits represents.
/// Although the bits are the same to the CPU, the type-system makes this distinction.
pub fn u16_vs_i16() {
    let a: u16 = 50115;
    let b: i16 = -15421;

    println!("the bits for the u16: {:016b} {}", a, a);
    println!("the bits for the i16: {:016b} {}", b, b);
}

/// We can take this process one step further.
/// What happens if we ask Rust to treat a bit pattern produced by one type as another?
/// The following listing provides an answer
pub fn interpret_a_float_as_an_int() {
    let a: f32 = 42.42;

    /// 'unsafe' tells the Rust compiler,
    /// “Stand back, I’ll take care of things from here. I’ve got this.”
    /// It’s a signal to the compiler that you have more context than it
    /// does to verify the correctness of the program. This doesn't mean
    /// that the code is unsafe, or that the borrow checker is disabled.
    /// It just means that the developer is taking responsibility for
    /// the safety of the code.
    let frankentype: u32 = unsafe {
        /// asks rust to naively reinterpret the bits of a float as an integer
        std::mem::transmute(a)
    };

    /// '{}' invokes the std::fmt::Display trait
    /// Side Note: '{:?}' invokes std::fmt:: Debug
    println!("float to int: {}", frankentype);

    /// The 032 reads as “left-pad with 32 zeros” and
    /// the right-hand b invokes the std::fmt::Binary trait
    println!("int as binary: {:032b}", frankentype);


    let b: f32 = unsafe {
        /// asks rust to naively reinterpret the integer back to a float
        std::mem::transmute(frankentype)
    };

    println!("int to float: {}", b);
    assert_eq!(a, b);
}

// Will result in a panic once it hits the
// maximum value of u16, which is 65535.
// Note: not meant to be called.
fn integer_overflow_example() {
    let mut i: u16 = 0;
    print!("{}..", i);

    loop {
        i += 1000;
        print!("{}..", i);
        if i % 10000 == 0 {
            print!{"\n"}
        }
    }
}

pub fn how_u16_bit_patterns_translate_to_a_fixed_number_of_integers () {
    let zero: u16 = 0b0000_0000_0000_0000;
    let one:  u16 = 0b0000_0000_0000_0001;
    let two:  u16 = 0b0000_0000_0000_0010;

    let sixty5_533: u16 = 0b1111_1111_1111_1101;
    let sixty5_534: u16 = 0b1111_1111_1111_1110;
    let sixty5_535: u16 = 0b1111_1111_1111_1111;

    print!("{}, {}, {}, ..., ", zero, one, two);
    println!("{}, {}, {}", sixty5_533, sixty5_534, sixty5_535);
}

/// Despite Rust's strengths, it is still possible to
/// write code that will break (if it doesn't panic).
#[allow(arithmetic_overflow)]
fn impossible_addition() {
    /// u8 can only hold values from 0 to 255.
    /// This will result in a panic, or the wrong
    /// value will be given.
    let (a, b) = (200, 200);
    let c: u8 = a + b;
    println!("200 + 200 = {}", c);
}

pub fn inspecting_endianness() {
    let big_endian: [u8; 4]    = [0xAA, 0xBB, 0xCC, 0xDD];
    let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];

    /// `std::mem::transmute()` instructs the compiler to
    /// interpret its argument as the type on the left.
    /// In this case, i32.
    let a: i32 = unsafe { std::mem::transmute(big_endian)    };
    let b: i32 = unsafe { std::mem::transmute(little_endian) };

    println!("{} vs {}", a, b);
}

// The bit at position 31 is 0, which means the number is positive.
pub fn isolating_and_decoding_the_sign_bit_of_a_32bit_floating_point_number(fp: f32) -> f32 {
    let n_bits: u32 = fp.to_bits();
    (n_bits >> 31) as f32
}

pub fn isolating_and_decoding_the_exponent_of_a_32bit_floating_point_number(fp: f32) -> f32 {
    let n_bits: u32 = fp.to_bits();
    let exponent_ = n_bits >> 23;
    let exponent_ = exponent_ & 0xff;
    (exponent_ as i32).sub(127) as f32
}

pub fn isolating_and_decoding_the_mantissa_of_a_32bit_floating_point_number(fp: f32) -> f32 {
    let n_bits: u32 = fp.to_bits();
    let mut mantissa: f32 = 1.0;

    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = n_bits & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf( i_ - 23.0 );
            mantissa += weight;
        }
    }

    mantissa
}

pub fn deconstruct_a_floating_point_number(fp: f32) -> () {
    let (sign, exp, frac) = to_parts(fp);
    let (sign_, exp_, mant) = decode(sign, exp, frac);
    let n_ = from_parts(sign_, exp_, mant);

    println!("{} -> {}", fp, n_);
    println!("field              |          as bits          | as real number");
    println!("sign               | {:01b}                         | {}", sign, sign_);
    println!("exponent           | {:08b}                  | {}", exp, exp_);
    println!("mantissa           | {:023b}   | {}", frac, mant);
}

fn to_parts(n: f32) -> (u32, u32, u32) {
    let bits = n.to_bits();

    let sign     = (bits >> 31) & 1;
    let exponent = (bits >> 23) & 0xff;
    let fraction =  bits & 0x7fffff ;

    (sign, exponent, fraction)
}

fn decode(
    sign: u32,
    exponent: u32,
    fraction: u32
) -> (f32, f32, f32) {
    let signed_1 = (-1.0_f32).powf(sign as f32);

    let mut mantissa: f32 = 1.0;
    let exponent = (exponent as i32) - BIAS;
    let exponent = RADIX.powf(exponent as f32);

    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = fraction & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf( i_ - 23.0 );
            mantissa += weight;
        }
    }

    (signed_1, exponent, mantissa)
}

fn from_parts(
    sign: f32,
    exponent: f32,
    mantissa: f32,
) -> f32 {
    sign *  exponent * mantissa
}
