mod using_a_boxed_trait_as_return_value;
mod using_enum_to_catch_upstream_error;

use std::net::{SocketAddr, UdpSocket};
use std::ops::Add;
use std::time::Duration;
use clap::{App, Arg};
use rand::random;
use trust_dns::op::{Message, MessageType, OpCode, Query};
use trust_dns::rr::{Name, RecordType};
use trust_dns::serialize::binary::{BinEncodable, BinEncoder};

fn main() {
    let app = App::new("resolve")
        .about("A simple to use DNS resolver")
        .arg(Arg::with_name("dns-server").short("s").default_value("1.1.1.1"))
        .arg(Arg::with_name("domain-name").required(true))
        .get_matches();

    let domain_name_raw = app.value_of("domain-name").map(|s| {
        if !s.starts_with("www.") {
            // s.to_string();

            return String::from("www.").add(s);
        }

        s.to_string()
    }).unwrap();

    let domain_name = Name::from_str_relaxed(&domain_name_raw).expect("Failed to parse domain name");

    let dns_server_raw = app.value_of("dns-server").expect("Failed to parse DNS server");
    let dns_server: SocketAddr = format!("{}:53", dns_server_raw)
        .parse()
        .expect("Invalid address");

    let mut request_as_bytes: Vec<u8> = Vec::with_capacity(512);
    let mut response_as_bytes: Vec<u8> = vec![0; 512];

    let mut msg = Message::new();

    msg
        .set_id(random::<u16>())
        .set_message_type(MessageType::Query)
        .add_query(Query::query(domain_name, RecordType::A))
        .set_op_code(OpCode::Query)
        .set_recursion_desired(true);

    let mut encoder = BinEncoder::new(&mut request_as_bytes);
    msg.emit(&mut encoder).unwrap();

    let localhost = UdpSocket::bind("0.0.0.0:0")
        .expect("cannot bind to local socket");

    let timeout = Duration::from_secs(3);

    localhost.set_read_timeout(Some(timeout))
        .expect("cannot set read timeout");

    localhost.set_nonblocking(false).unwrap();

    let _amt = localhost.send_to(&request_as_bytes, dns_server)
        .expect("socket misconfigured");

    let (_amt, _remote) = localhost.recv_from(&mut response_as_bytes).expect("timeout reached");

    let dns_message = Message::from_vec(&response_as_bytes).expect("cannot parse response");

    for answer in dns_message.answers() {
        if answer.record_type() == RecordType::A {
            let resource = answer.rdata();
            let ip = resource.to_ip_addr().expect("invalid IP address received");

            println!("{}", ip.to_string());
        }
    }
}
