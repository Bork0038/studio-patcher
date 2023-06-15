use super::super::{ Packet, NetworkStream };

pub fn deserialize( mut stream: NetworkStream ) -> Option<Packet> {
    stream.ignore_bytes( 1 );

    Some(
        Packet::ID_MARKER {
            id: 0x84,
			len: stream.get_data().len(),
            marker_id: stream.read_be()
        }
    )
}