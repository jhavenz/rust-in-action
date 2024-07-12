use crate::ownership_using_copy_and_clone::own_with_copy;
use crate::wrappers::run_with_ownership_wrappers;

mod failed_ownership;
mod ownership_using_copy_and_clone;
mod wrappers;

#[allow(unused_variables)]

struct GroundStation;

impl GroundStation {
    fn send(&self, mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg);
    }

    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat { id: sat_id }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct CubeSat {
    id: u64,
}

impl Copy for CubeSat {}

impl Clone for CubeSat {
    fn clone(&self) -> CubeSat {
        CubeSat { id: self.id }
    }
}

impl CubeSat {
    #[allow(dead_code)]
    fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

impl Clone for StatusMessage {
    fn clone(&self) -> StatusMessage {
        match self {
            StatusMessage::Ok => StatusMessage::Ok,
        }
    }
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }

        None
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Message {
    to: u64,
    content: String,
}

impl Message {
    #[allow(dead_code)]
    fn new(to: u64, content: String) -> Message {
        Message { to, content }
    }
}

#[allow(dead_code)]
fn check_status_taking_ownership(sat_id: CubeSat) {
    println!("{:?}: {:?}", sat_id, StatusMessage::Ok);
}

#[allow(dead_code)]
fn check_status(sat: CubeSat) -> CubeSat {
    println!("{:?}: {:?}", sat.id, StatusMessage::Ok);

    sat
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1,2,3]
}

/// Uses 'short-lived' variables to demonstrate ownership transfer
/// between functions.
///
/// The 'Mailbox' as a way to transfer ownership of the 'Message' between
/// the 'GroundStation' and 'CubeSat'.
fn main() {
    let mut mail = Mailbox { messages: vec![] };

    let base = GroundStation {};

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        base.connect(sat_id);
        let msg = Message { to: sat_id, content: String::from("hello") };

        base.send(&mut mail, msg);
    }

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = sat.recv(&mut mail);

        println!("{:?}: {:?}", sat, msg);
    }

    own_with_copy();

    run_with_ownership_wrappers();
}
