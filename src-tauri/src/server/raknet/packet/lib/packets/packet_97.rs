use super::super::{ Packet, NetworkStream };

pub fn deserialize( mut stream: NetworkStream ) -> Option<Packet> {
    stream.ignore_bytes( 1 );

    let compressed_len: u32 = stream.read_le();
    let decompressed_len: u32 = stream.read_le();

    // println!("{} {} {}", compressed_len, decompressed_len, stream.get_data().len());
    None
}