use base64::decode;
use std::time::{ Duration, SystemTime };

pub struct Session {
    pub packets: Vec<SessionPacket>
}

pub struct SessionPacket {
    pub timing: u128,
    pub data: Box<Vec<u8>>
}

impl Session {

    pub fn new<S: AsRef<[u8]>>( bytes: S ) -> Self {
        let mut packets = Vec::new();
        let bytes = bytes.as_ref();

        let mut idx = 0;
        let mut last_time = 0;
        let mut last_timing = 0;
        let mut first = true;

        loop {
            let packet_len = u32::from_le_bytes(
                bytes[ idx .. idx + 4 ].try_into().unwrap()
            );
            idx += 4;

            let encoded_len = u32::from_le_bytes(
                bytes[ idx .. idx + 4 ].try_into().unwrap()
            ) as usize;
            idx += 4;
            
            let encoded_data = &bytes[ idx .. idx + encoded_len ];
            let decoded_packet: Box<Vec<u8>> = Box::new( decode( encoded_data ).unwrap() );
            idx += encoded_len;
        
            let timing = u128::from_le_bytes(
                decoded_packet[ 0..16 ].try_into().unwrap()
            );

            last_time = timing - last_timing;
            last_timing = timing;

            let packet = SessionPacket {
                timing: if first { 0 } else { last_time },
                data: Box::new(
                    decoded_packet[ 16..packet_len as usize ].to_vec()
                )
            };
            first = false;
            
            packets.push( packet );

            if idx == bytes.len() {
                break;
            }
        }

        Session { packets }
    }

}