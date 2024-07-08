use std::mem::drop;
use std::time::Instant;

/**
The Heap:

The word “heap” implies disorganization. A closer analogy would be warehouse space in some
medium-sized business. As deliveries arrive (as variables are created), the warehouse makes
space available. As the business carries out its work, those materials are used, and the
warehouse space can now be made available for new deliveries. At times, there are gaps and
perhaps a bit of clutter. But overall, there is a good sense of order.

Another mistake is that the heap has no relationship to the data structure that is also
known as a heap. That data structure is often used to create priority queues. It’s an
incredibly clever tool in its own right, but right now it’s a complete distraction. The heap
is not a data structure. It’s an area of memory.
 */

pub fn heap_example() -> i32 {
    let now = Instant::now();
    let a: i32 = 40;
    println!("Time to create a_i32: {:?}", now.elapsed().as_nanos());
    let b: Box<i32> = Box::new(41);
    println!("Time to create b: Box<i32>: {:?}", now.elapsed().as_nanos());

    // 'b' cannot be accessed without using the (unary) * operator.
    a + *b
}

// Note: Integers and other types implementing Copy are unaffected by drop.
pub fn allocating_and_deallocating_memory_on_the_heap() {
    let a = Box::new(1);
    let b = Box::new(1);
    let c = Box::new(1);

    let result1 = *a + *b + *c;

    drop(a);
    let d = Box::new(1);
    let result2 = *b + *c + *d;

    println!("allocating and deallocating result: {} {}", result1, result2);
}
