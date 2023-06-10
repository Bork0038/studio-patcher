use num_derive::{ FromPrimitive, ToPrimitive };
use serde::{ Serialize, Deserialize };
use num_traits::{ FromPrimitive, ToPrimitive };

#[repr(u8)]
#[derive(FromPrimitive, ToPrimitive, PartialEq, Clone, Serialize, Deserialize)]
pub enum Opcode {
    IncomingPackets,
    OutgoingPackets
}

#[repr(u8)]
#[derive(FromPrimitive, ToPrimitive, PartialEq, Clone, Serialize, Deserialize)]
pub enum PacketType {
    StudioClient,
    TestClient,
    Server
}