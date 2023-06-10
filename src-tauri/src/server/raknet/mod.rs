mod lib;
pub use lib::*;

use tauri::{ Manager, AppHandle };
use rouille::Request;
use std::io::Read;
use super::super::stream::NetworkStream;
use num_traits::FromPrimitive;

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
 
    let packet = stream.read_to_end();
    println!("{:02X}", packet[0]);
}