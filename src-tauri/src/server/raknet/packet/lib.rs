mod packets;

use super::{ Packet, NetworkStream };
use std::convert::From;

impl Packet {

    pub fn deserialize( packet_data: &Vec<u8> ) -> Option<Packet> {
        let packet_id = if packet_data[ 0 ] == 0x83 {
            packet_data[ 0 ] as u16 * 0x100 | packet_data[ 1 ] as u16
        } else {
            packet_data[ 0 ] as u16
        }; 

        let stream = NetworkStream::from( packet_data.to_owned() );
        match packet_id {
            0x81 => packets::packet_81::deserialize( stream ),
            0x84 => packets::packet_84::deserialize( stream ),
            0x93 => packets::packet_93::deserialize( stream ),
            0x97 => packets::packet_97::deserialize( stream ),
            _ => None
        }
    }

}