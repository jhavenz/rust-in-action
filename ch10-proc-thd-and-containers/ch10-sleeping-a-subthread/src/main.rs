use std::{thread, time};

fn main() {
    let start = time::Instant::now();

    let handler = thread::spawn(|| {
        let pause = time::Duration::from_millis(300);
        thread::sleep(pause.clone());
        println!("in thread: {}", std::process::id());
    });

    handler.join().unwrap();

    let finish = time::Instant::now();

    println!("Time elapsed: {:?}", finish.duration_since(start));
}
