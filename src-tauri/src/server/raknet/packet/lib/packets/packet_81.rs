use super::super::{ Packet, NetworkStream };

pub fn deserialize( mut stream: NetworkStream ) -> Option<Packet> {
    stream.ignore_bytes( 1 );

    let streaming_enabled = stream.read_bool();
    let filtering_enabled = stream.read_bool();
    let third_party_asset_allowed = stream.read_bool();
    let third_party_purchase_allowed = stream.read_bool();
    let third_party_teleport_allowed = stream.read_bool();
    
    let peer_id: u32 = stream.read_le();

    Some(
        Packet::ID_SET_GLOBALS { 
            id: 0x81,
            len: stream.get_data().len(), 
            streaming_enabled, 
            filtering_enabled, 
            third_party_asset_allowed, 
            third_party_purchase_allowed,
            third_party_teleport_allowed, 
            peer_id
        }
    )
}