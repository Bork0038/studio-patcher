use num_derive::{ FromPrimitive, ToPrimitive };
use serde::{ Serialize, Deserialize };
use num_traits::{ FromPrimitive, ToPrimitive };
use std::fmt;

#[repr(u8)]
#[derive(FromPrimitive, ToPrimitive, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum Opcode {
    IncomingPackets,
    OutgoingPackets
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(u8)]
#[derive(FromPrimitive, ToPrimitive, PartialEq, Clone, Serialize, Deserialize)]
pub enum PacketType {
    StudioClient,
    TestClient,
    Server
}