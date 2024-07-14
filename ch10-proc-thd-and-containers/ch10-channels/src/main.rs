use std::thread;
use crossbeam::{select, unbounded};
use crate::ConnectivityCheck::*;

#[derive(Debug)]
enum ConnectivityCheck {
    Ping,
    Pong,
    Pang
}

fn main() {
    // create_a_channel();
    create_multiple_channels();
}

fn create_a_channel() {
    let (tx, rx) = unbounded();

    thread::spawn(move || {
        tx.send(42).unwrap();
    });

    select! {
        recv(rx) -> msg => {
            println!("Received: {:?}", msg);
        }
    }
}

fn create_multiple_channels () {
    let n_messages = 3;
    let (requests_tx, requests_rx) = unbounded();
    let (responses_tx, responses_rx) = unbounded();

    thread::spawn(move || loop {
        match requests_rx.recv().unwrap() {
            Pong => eprintln!("unexpected pong response"),
            Ping => responses_tx.send(Pong).unwrap(),
            Pang => return,
        }
    });

    for _ in 0..n_messages {
        requests_tx.send(Ping).unwrap();
    }
    requests_tx.send(Pang).unwrap();

    for _ in 0..n_messages {
        select! {
           recv(responses_rx) -> msg => println!("{:?}", msg),
        }
    }
}
