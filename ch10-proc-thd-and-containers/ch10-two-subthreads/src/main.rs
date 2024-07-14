use std::{thread, time};

fn main() {
    let start = time::Instant::now();

    let handler_1 = thread::spawn(move || {
        let pause = time::Duration::from_millis(300);
        thread::sleep(pause.clone());
    });

    let handler_2 = thread::spawn(move || {
        let pause = time::Duration::from_millis(300);
        thread::sleep(pause.clone());
    });

    let finish_1 = time::Instant::now();
    handler_1.join().unwrap();
    let finish_2 = time::Instant::now();
    handler_2.join().unwrap();

    let done = time::Instant::now();

    println!("Time elapsed: {:?}, {:?}, {:?}, {:?}", finish_1.duration_since(start), finish_2.duration_since(start), finish_2.duration_since(finish_1), done.duration_since(start));
}
