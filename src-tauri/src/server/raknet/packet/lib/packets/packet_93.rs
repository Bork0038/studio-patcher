use super::super::{ Packet, NetworkStream };

pub fn deserialize( mut stream: NetworkStream ) -> Option<Packet> {
    stream.ignore_bytes( 2 );

    let flag_count: u16 = stream.read_be();

    let mut fflags = Vec::new();
    for _i in 0..flag_count {
        let flag_name = stream.read_string_be::<u16>(); // pick a fucking endianness and stick to it please
        let flag_value = stream.read_string_be::<u16>();

        fflags.push(
            ( flag_name, flag_value )
        );
    }

    Some(
        Packet::ID_DICTIONARY_FORMAT { id: 0x93, fflags }
    )
}