mod lib;
mod packet;

pub use lib::*;
pub use packet::Packet;

use serde_derive::{Deserialize, Serialize};
use tauri::{ Manager, AppHandle };
use rouille::Request;
use std::io::Read;
use super::super::stream::NetworkStream;
use num_traits::FromPrimitive;

#[derive(Deserialize, Serialize, Clone)]
pub struct PacketTransfer {
    opcode: Opcode,
    packet_type: PacketType,
    address: SystemAddress,
    packet: Packet
}

pub fn handle_connection( app: AppHandle, req: &Request ) {
    if req.method() != "POST" { return };

    let window = match app.get_window( "/raknet" ) {
        Some(window) => window,
        None => return
    };

    let mut data = match req.data() {
        Some(data) => data,
        None => return
    };
    
    let mut buf = Vec::new();
    data.read_to_end( &mut buf ).unwrap();

    let mut stream = NetworkStream::from( buf );

    let opcode = Opcode::from_u8( stream.read_byte() ).unwrap();
    let packet_type = PacketType::from_u8( stream.read_byte() ).unwrap();
    let address: SystemAddress = stream.read();
 
    let packet_data = stream.read_to_end().to_vec();
    if let Some(packet) = Packet::deserialize( &packet_data ) {
        println!("{}", packet);
        let packet_transfer = PacketTransfer { opcode, packet_type, address, packet };

        window.emit( "packet-data", packet_transfer ).unwrap();
    } else {    
        let packet_id = if packet_data[ 0 ] == 0x83 {
            packet_data[ 0 ] as u16 * 0x100 | packet_data[ 1 ] as u16
        } else {
            packet_data[ 0 ] as u16
        }; 

        // println!("{} UNKNOWN_PACKET #{:X}", opcode, packet_id)
    }
}