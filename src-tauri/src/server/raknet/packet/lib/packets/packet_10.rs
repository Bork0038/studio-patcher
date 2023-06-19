use super::super::{ Packet, NetworkStream };
use super::lib::ShortAddress;

pub fn deserialize( mut stream: NetworkStream ) -> Option<Packet> {
    stream.ignore_bytes( 1 );

    // let version = stream.read_byte();
    let address: ShortAddress = stream.read();
    let system_index = stream.read_be();

    let mut address_list: Vec<ShortAddress> = Vec::new();
    for _ in 0..10 {
        address_list.push( stream.read() );
    }

    let send_ping_time = stream.read_be();
    let send_pong_time = stream.read_be();

    Some( 
        Packet::ID_CONNECTION_REQUEST_ACCEPTED { 
            id: 0x10,
            len: stream.get_data().len(), 
            address, 
            system_index, 
            address_list, 
            send_ping_time, 
            send_pong_time 
        }
    )
}