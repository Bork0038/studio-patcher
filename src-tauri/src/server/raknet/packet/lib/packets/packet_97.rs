use super::super::{ Packet, NetworkStream };
use serde_derive::{Deserialize, Serialize};
use zstd::stream::Decoder;
use std::io;
pub use super::lib::*;

fn read_varint_string( stream: &mut NetworkStream ) -> String {
    let len = stream.read_varint32();
    let bytes = stream.read_bytes( len );

    String::from_utf8( bytes ).unwrap()
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
        let name = read_varint_string( &mut decompressed_stream );
        let size = decompressed_stream.read_byte();

        enums.push( NetworkEnum { name, network_id: network_id as u16, size });
    }

    let class_len = decompressed_stream.read_varint32();
    let _prop_len = decompressed_stream.read_varint32();
    let _event_len = decompressed_stream.read_varint32();

    let mut classes = Vec::new();
    let mut optimized_strings = Vec::new();
    let mut content_prefixes = Vec::new();

    let mut class_index = 0;
    let mut prop_index = 0;
    let mut event_index = 0;

    {
        for i in 0..class_len {
            let class_name = read_varint_string( &mut decompressed_stream );

            let mut class_props = Vec::new();
            let mut class_events = Vec::new();

            for _ in 0..decompressed_stream.read_varint32() {
                let name = read_varint_string( &mut decompressed_stream );

                let prop_type = NetworkPropertyType::from_u8( decompressed_stream.read_byte() ).unwrap();
                let prop_enum_id: Option<u16> = if prop_type == NetworkPropertyType::Enum_VarInt {
                    Some( decompressed_stream.read_be() )
                } else {
                    None
                };
               
                class_props.push( 
                    NetworkProperty { 
                        name, 
                        prop_type, 
                        prop_enum_id, 
                        network_id: prop_index
                    } 
                );
                prop_index += 1;
            }

            decompressed_stream.ignore_bytes( 2 ); // u16 idk what this does don't need it   
        
            for _ in 0..decompressed_stream.read_varint32() {
                let name = read_varint_string( &mut decompressed_stream );
                let mut arguments = Vec::new();

                for _ in 0..decompressed_stream.read_varint32() {
                    let argument_type = NetworkPropertyType::from_u8( decompressed_stream.read_byte() ).unwrap();
                    let argument_enum_id = decompressed_stream.read_be();
                    
                    arguments.push( 
                        NetworkArgument {
                            argument_enum_id,
                            argument_type
                        }
                    );
                }

                class_events.push(
                    NetworkEvent {
                        arguments,
                        name,
                        network_id: event_index
                    }
                );
                event_index += 1;
            }

            classes.push(
                NetworkClass {
                    name: class_name,
                    network_id: class_index,
                    properties: class_props,
                    events: class_events
                }
            );
            
            class_index += 1;
        }

        for _ in 0..decompressed_stream.read_varint32() {
            content_prefixes.push(
                read_varint_string( &mut decompressed_stream )
            );
        }

        for _ in 0..decompressed_stream.read_varint32() {
            optimized_strings.push( 
                read_varint_string( &mut decompressed_stream )
            );
        }
    }

    let schema = NetworkSchema {
        enums,
        classes,
        content_prefixes,
        optimized_strings
    };

    Some( 
        Packet::ID_SCHEMA_SYNC_V2 {
            id: 0x97,
            len: compressed_len as usize,
            schema
        }
    )
}