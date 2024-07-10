use std::io::prelude::*;

const BYTES_PER_LINE: usize = 16;

const INPUT: &'static [u8] = br#"
  fn main() {
      println!("Hello, world!");
  }"#;

pub fn run() -> std::io::Result<()> {
    let mut buffer: Vec<u8> = vec!();

    INPUT.read_to_end(&mut buffer)?; // Reads our input and inserts it into our internal buffer

    let mut position_in_input = 0;
    for line in buffer.chunks(BYTES_PER_LINE) {
        print!("[0x{:08x}] ", position_in_input); // Writes the current position with up to 8 left-padded zeros
        for byte in line {
            print!("{:02x} ", byte);
        }
        println!();
        position_in_input += BYTES_PER_LINE;
    }

    Ok(())
}
