fn run_with_seg_fault_b() {
    let mut n_nonzero = 0;

    for i in 1..10000 {
        // Fails when trying to reference a non-existing memory address (index 1).
        let ptr = i as *const u8;
        let byte_at_addr = unsafe { *ptr };

        if byte_at_addr != 0 {
            n_nonzero += 1;
        }
    }

    println!("non-zero bytes in memory: {}", n_nonzero);
}

fn run_with_seg_fault_a() {
    let mut n_nonzero = 0;

    for i in 0..10000 {
        // Fails when trying to reference a null pointer (index 0).
        let ptr = i as *const u8;
        let byte_at_addr = unsafe { *ptr };

        if byte_at_addr != 0 {
            n_nonzero += 1;
        }
    }

    println!("non-zero bytes in memory: {}", n_nonzero);
}
