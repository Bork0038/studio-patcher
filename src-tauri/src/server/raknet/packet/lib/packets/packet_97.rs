use super::super::{ Packet, NetworkStream };
use serde_derive::{Deserialize, Serialize};
use zstd::stream::Decoder;
use std::io;

#[derive(Deserialize, Serialize)]
pub struct NetworkEnum {
    name: String,
    size: u8,
    network_id: u16
}

#[derive(Deserialize, Serialize)]
pub struct NetworkProperty {
    name: String,
    network_id: u16
}

#[derive(Deserialize, Serialize)]
pub struct NetworkClass {
    name: String,
    network_id: u16,
    properties: Vec<NetworkProperty>
}

pub fn deserialize( mut stream: NetworkStream ) -> Option<Packet> {
    stream.ignore_bytes( 1 );

    let compressed_len: u32 = stream.read_be();
    let decompressed_len: u32 = stream.read_be();

    let mut decompressed_stream = {
        let mut decoder = Decoder::new( stream.read_to_end() ).unwrap();
        let mut data = Vec::new();
        
        io::copy( &mut decoder, &mut data ).unwrap();

        NetworkStream::from( data )
    };

    if decompressed_stream.get_data().len() != decompressed_len as usize {
        panic!("Failed to decompress network schema");
    }
    
    let enums_len = decompressed_stream.read_varint32();
    let mut enums = Vec::new();
    for network_id in 0..enums_len {
        let len = decompressed_stream.read_varint32();

        let name = {
            let bytes = decompressed_stream.read_bytes( len );

            String::from_utf8( bytes ).unwrap()
        };

        let size = decompressed_stream.read_byte();

        enums.push( NetworkEnum { name, network_id: network_id as u16, size });
    }

    let class_len = decompressed_stream.read_varint32();
    let property_len = decompressed_stream.read_varint32();
    let event_len = decompressed_stream.read_varint32();

    let mut classes = Vec::new();
    // let mut props = Vec::new();
    // let mut events = Vec::new();
    {
        for i in 0..class_len {
            let class_name_len = decompressed_stream.read_varint32();

            let class_name = {
                let bytes = decompressed_stream.read_bytes( class_name_len );
    
                String::from_utf8( bytes ).unwrap()
            };

            let class_network_id = decompressed_stream.read_le();
            let mut properties = Vec::new();

            classes.push(
                NetworkClass {
                    name: class_name,
                    network_id: class_network_id,
                    properties
                }
            );
            
            break;
        }
    }

    None
}