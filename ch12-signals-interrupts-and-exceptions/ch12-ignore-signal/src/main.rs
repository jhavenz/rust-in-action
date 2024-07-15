use libc::{raise, SIG_DFL, SIG_IGN, signal, SIGTERM};

fn main() {
    unsafe {
        signal(SIGTERM, SIG_IGN);
        raise(SIGTERM);
    }

    println!("ok");

    unsafe {
        signal(SIGTERM, SIG_DFL);
        raise(SIGTERM);
    }

    println!("not ok");
}
