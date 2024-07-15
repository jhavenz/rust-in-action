
fn noop() {
    // do nothing
}

fn main() {
    // casts a function pointer to a 'usize' integer
    let fn_ptr = noop as usize;

    // casts the 'usize' integer back to a function pointer, or more formally,
    // "a const pointer to a function that takes no arguments and returns 'unit' <()>"
    let typed_fn_ptr = noop as *const fn() -> ();

    println!("noop as usize: 0x:{:x}", fn_ptr);
    println!("noop as *const T: {:p}", typed_fn_ptr);
}
