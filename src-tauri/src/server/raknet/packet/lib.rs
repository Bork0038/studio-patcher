pub mod packets;

use super::{ Packet, NetworkStream };

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

            0x83_04 => packets::packet_83_04::deserialize( stream ),
            0x83_05 => packets::packet_83_05::deserialize( stream ),
            0x83_06 => packets::packet_83_06::deserialize( stream ),
            0x83_10 => packets::packet_83_10::deserialize( stream ),
            _ => None
        }
    }

}