use super::super::{ Packet, NetworkStream };
use std::collections::HashMap;

pub fn deserialize( mut stream: NetworkStream ) -> Option<Packet> {
    stream.ignore_bytes( 2 );

    let flag_count: u16 = stream.read_be();

    let mut fflags = HashMap::new();
    for _i in 0..flag_count {
        let flag_name = stream.read_string_be::<u16>(); // pick a fucking endianness and stick to it please
        let flag_value = stream.read_string_be::<u16>();

        fflags.insert( flag_name, flag_value );
    }

    Some(
        Packet::ID_SFFLAG_SYNC { 
            id: 0x93,
			len: stream.get_data().len(), 
            fflags 
        }
    )
}