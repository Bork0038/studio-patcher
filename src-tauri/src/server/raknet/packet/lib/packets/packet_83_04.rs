use super::super::{ Packet, NetworkStream };

pub fn deserialize( mut stream: NetworkStream ) -> Option<Packet> {
    stream.ignore_bytes( 2 );

    Some(
        Packet::ID_REPLIC_MARKER {
            id: 0x83_04,
            marker_id: stream.read_be()
        }
    )
}