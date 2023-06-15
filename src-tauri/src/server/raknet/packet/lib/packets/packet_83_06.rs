use super::super::{ Packet, NetworkStream };

pub fn deserialize( mut stream: NetworkStream ) -> Option<Packet> {
    stream.ignore_bytes( 2 );

    let type_and_version = stream.read_byte();
    let time = stream.read_be();
    let first_serialize_out_of_time = stream.read_be();

    let (
        peer_ping_arrival_local_read_time,
        peer_reply_queue_time,
        peer_reply_serialize_time,
        peer_last_known_bcs_queue_time,
        peer_last_known_recv_to_pop_time,
        peer_last_known_pop_to_read_time,
        render_fps,
        physics_fps,
        heartbeat_fps
    ) = if ( type_and_version & 1 ) != 0 {
        (
            Some( stream.read_be() ),
            Some( stream.read_be() ),
            Some( stream.read_be() ),
            Some( stream.read_be() ),
            Some( stream.read_be() ),
            Some( stream.read_be() ),
            Some( stream.read_be() ),
            Some( stream.read_be() ),
            Some( stream.read_be() ),
        )
    } else {    
        ( None, None, None, None, None, None, None, None, None )
    };      

    Some(
        Packet::ID_REPLIC_PING_BACK { 
            id: 0x83_06,
            len: stream.get_data().len(),
            type_and_version,
            time,
            first_serialize_out_of_time,
            peer_ping_arrival_local_read_time,
            peer_reply_queue_time,
            peer_reply_serialize_time,
            peer_last_known_bcs_queue_time,
            peer_last_known_recv_to_pop_time,
            peer_last_known_pop_to_read_time,
            render_fps,
            physics_fps,
            heartbeat_fps,
            send_stats: stream.read_be(),
            extra_stats: stream.read_be(),
        }
    )
}