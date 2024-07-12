use std::io::{Error, Write};
use std::net::TcpStream;
use rand::{random};
use trust_dns::op::{Message, MessageType, OpCode, Query};
use trust_dns::rr::{Name, RecordType};

fn main() -> std::io::Result<()> {
    // tcp_request()?;

    dns_lookup();

    Ok(())
}

fn tcp_request() -> Result<(), Error> {
    let host = "jsonplaceholder.typicode.com:80";

    let mut conn = TcpStream::connect(host)?;

    conn.write_all(b"GET / HTTP/1.0")?;
    conn.write_all(b"\r\n")?;

    conn.write_all(b"Host: jsonplaceholder.typicode.com")?;
    conn.write_all(b"\r\n\r\n")?;

    std::io::copy(&mut conn, &mut std::io::stdout())?;

    Ok(())
}

fn dns_lookup() {
    let domain_name = Name::from_ascii("www.rust-lang.org").unwrap();

    let mut msg = Message::new();

    msg
        .set_id(random::<u16>())
        .set_message_type(MessageType::Query)
        .add_query(Query::query(domain_name, RecordType::A))
        .set_op_code(OpCode::Query)
        .set_recursion_desired(true);

    println!("Message: {:?}", msg);
}
