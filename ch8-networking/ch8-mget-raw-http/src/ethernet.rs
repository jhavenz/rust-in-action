use std::fmt;
use std::fmt::{Display, Formatter};
use rand::{RngCore, thread_rng};
use smoltcp::wire;

#[derive(Debug)]
pub struct MacAddress([u8; 6]);

impl Display for MacAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let octet = &self.0;

        write!(f,
           "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
           octet[0], octet[1], octet[2], octet[3], octet[4], octet[5]
        )
    }
}

impl Into<wire::EthernetAddress> for MacAddress {
    fn into(self) -> wire::EthernetAddress {
        wire::EthernetAddress { 0: self.0 }
    }
}

impl MacAddress {
    pub fn new() -> MacAddress {
        let mut octets: [u8; 6] = [0; 6];

        thread_rng().fill_bytes(&mut octets);

        octets[0] |= 0b_0000_0011;

        MacAddress(octets)
    }

    pub fn is_local(&self) -> bool {
        (self.0[0] & 0b_0000_0010) == 0b_0000_0010
    }

    pub fn is_unicast(&self) -> bool {
        (self.0[0] & 0b_0000_0011) == 0b_0000_0001
    }
}
