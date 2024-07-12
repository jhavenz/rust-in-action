use std::mem::size_of;

// A smart pointer type that reads from its pointer location without needing to copy it first
use std::borrow::Cow;

// CStr is a C-like string type that allows Rust to read in zero-terminated strings.
use std::ffi::CStr;

// c_char, a type alias for Rust’s i8 type, presents the possibility of a platform-specific nuances.
use std::os::raw::c_char;

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

pub fn mimicking_pointers_with_references() {
    let a = 42;
    let b = &B;
    let c = &C;

    println!("a: {}, b: {:p}, c: {:p}", a, b, c);
}

pub fn comparing_references_and_a_box_of_t_to_several_types() {
    // usize is the memory address size for the CPU the code is compiled for. That CPU is called the compile target.
    let a: usize = 42;

    // &[u8; 10] reads as “a reference to an array of 10 bytes.” The array is located in static memory, and the reference itself (a pointer of width usize bytes) is placed on the stack.
    let b: &[u8; 10] = &B;

    // The Box<[u8]> type is a boxed byte slice. When we place values inside a box, ownership of the value moves to the owner of the box.
    let c: Box<[u8]> = Box::new(C);

    println!("--- usize ---");
    println!("(an unsigned integer):");
    println!("  location: {:p}", &a);
    println!("  size:     {:?} bytes", size_of::<usize>());
    println!("  value:    {:?}", a);
    println!();

    println!("--- &[u8; 10] ---");
    println!("(a reference to B):");
    println!("  location:  {:p}", &b);
    println!("  size:      {:?} bytes", size_of::<&[u8; 10]>());
    println!("  points to: {:p}", b);
    println!();

    println!("--- Box<[u8]> ---");
    println!("(a 'box' for C):");
    println!("  location:  {:p}", &c);
    println!("  size:      {:?} bytes", size_of::<Box<[u8]>>());
    println!("  points to: {:p}", c);
    println!();

    println!("--- [u8; 10] ---");
    println!("(an array of 10 bytes):");
    println!("  location: {:p}", &B);
    println!("  size:     {:?} bytes", size_of::<[u8; 10]>());
    println!("  value:    {:?}", B);
    println!();

    println!("--- [u8; 11] ---");
    println!("(an array of 11 bytes):");
    println!("  location: {:p}", &C);
    println!("  size:     {:?} bytes", size_of::<[u8; 11]>());
    println!("  value:    {:?}", C);
}

pub fn printing_strings_provided_by_external_sources() {
    // Introduces each of the variables so that these are accessible from println! later. If we created b and c within the unsafe block, these would be out of scope later.
    let a = 42;

    // String is a smart pointer type that holds a pointer to a backing array and a field to store its size.
    let b: String = String::from("foo");

    // Cow accepts a type parameter for the data it points to; str is the type returned by CStr.to_string_lossy(), so it is appropriate here.
    let c: Cow<str>;

    unsafe {
        // References cannot be cast directly to *mut T, the type required by String::from_raw_parts(). But *const T can be cast to *mut T, leading to this double cast syntax.
        // let b_ptr = &B as *const u8 as *mut u8;

        // String::from_raw_parts() accepts a pointer (*mut T) to an array of bytes, a size, and a capacity parameter.
        // b = String::from_raw_parts(b_ptr, 10, 10);

        // Converts a *const u8 to a *const i8, aliased to c_char. The conversion to i8 works because we remain under 128, following the ASCII standard.
        // raw pointers are denoted as *const T and *mut T for immutable and mutable raw pointers, respectively.
        // Even though each is a single type, these contain three tokens: '*', 'const' or 'mut' & their type 'T'.
        // A raw pointer (immutable) to a String, looks like `*const String`.
        // A raw pointer (mutable) to a String, looks like `*mut String`.
        let c_ptr = &C as *const u8 as *const c_char;

        // Conceptually, CStr::from_ptr() takes responsibility for reading the pointer until it reaches 0; then it generates Cow from the result
        c = CStr::from_ptr(c_ptr).to_string_lossy();
    }

    println!("a: {}, b: {}, c: {}", a, b, c);
}

/**
    The terms pointer and memory address are sometimes used interchangeably.
    These are integers that represent a location in virtual memory.
    From the compiler’s point of view, though, there is one important difference.
    Rust’s pointer types `*const T` and `*mut T` always point to the starting byte of T,
    and these also know the width of type T in bytes. A memory address might refer to anywhere in memory.
*/
pub fn creating_a_raw_pointer () {
    let a: i64 = 42;

    // Casts a reference to the variable a (&a) to a constant raw pointer i64 (*const i64)
    let a_ptr = &a as *const i64;

    // Prints the value of the variable a (42) and its address in memory (0x7ff...)
    println!("a: {} ({:p})", a, a_ptr);
}

/**
    An i64 is 8-bytes wide (64 bits ÷ 8 bits per byte).
    Therefore, if an i64 is stored at address 0x7fffd,
    then each of the bytes between 0x7ffd..0x8004 must
    be fetched from RAM to recreate the integer’s value.
    The process of fetching data from RAM from a pointer
    is known as dereferencing a pointer. The following
    function identifies a value’s address by casting a
    reference to it as a raw pointer via std::mem::transmute.
*/
pub fn identifying_a_values_address() {
    let a: i64 = 42;
    let a_ptr = &a as *const i64;
    let a_addr: usize = unsafe {
        std::mem::transmute(a_ptr)
    };

    println!("a: {} ({:p}...0x{:x})", a, a_ptr, a_addr + 7);
}

// For example only, not meant to be executed
fn example_of_dereferencing_a_raw_pointer() {
    // The address of a Vec<String> is stored in the ptr variable.
    let ptr = 42 as *const Vec<String>;

    unsafe {

        let new_addr = ptr.offset(4);
        println!("{:p} -> {:p}", ptr, new_addr);
    }
}
