use std::{process, time};
use std::thread::sleep;

// Used to send `kill -SIGSTOP <pid>` to the process
// then use `kill -SIGCONT <pid>` to resume
fn main() {
    let delay = time::Duration::from_secs(1);

    let pid = process::id();
    println!("{}", pid);

    for i in 1..=60 {
        sleep(delay);
        println!(". {}", i);
    }
}
