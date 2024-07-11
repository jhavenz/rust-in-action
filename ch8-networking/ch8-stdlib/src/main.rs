mod dns;

use std::io::Write;
use std::net::TcpStream;
use rand::{random, thread_rng};
use trust_dns::op::{Message, MessageType, OpCode, Query};
use trust_dns::rr::RecordType;

fn main() -> std::io::Result<()> {
    let host = "jsonplaceholder.typicode.com:80";

    let mut conn = TcpStream::connect(host)?;

    conn.write_all(b"GET / HTTP/1.0")?;
    conn.write_all(b"\r\n")?;

    conn.write_all(b"Host: jsonplaceholder.typicode.com")?;
    conn.write_all(b"\r\n\r\n")?;

    std::io::copy(&mut conn, &mut std::io::stdout())?;

    dns_lookup();

    Ok(())
}

fn dns_lookup() {
    let domain_name = "www.rust-lang.org";
    let mut msg = Message::new();

    msg
        .set_id(rand::random::<u16>())
        .set_message_type(MessageType::Query)
        .add_query(Query::query(domain_name, RecordType::A))
        .set_op_code(OpCode::Query)
        .set_recursion_desired(true);

}
