use super::super::{ Packet, NetworkStream };
use num_traits::{ FromPrimitive, ToPrimitive };
use num_derive::{ FromPrimitive, ToPrimitive };
use serde_derive::{ Serialize, Deserialize };

#[derive(Debug, FromPrimitive, Serialize, Deserialize, Clone)]
pub enum TagItemType {
    REPLICATED_FIRST_FINISHED_TAG = 0x0C,
    TOP_REPLICATION_CONTAINER_FINISHED_TAG = 0x0D,
    ANIMATION_CYCLE_COMPLETED_TAG = 0x0E,
    STREAMING_MIN_REGIONS_COMPLETED_TAG = 0x0F
}

pub fn deserialize( mut stream: NetworkStream ) -> Option<Packet> {
    stream.ignore_bytes( 2 );

    let tag_id: i32 = stream.read_be();
    Some(
        Packet::ID_REPLIC_TAG {
            id: 0x83_10,
			len: stream.get_data().len(),
            tag_id: TagItemType::from_u8( tag_id as u8 ).unwrap()
        }
    )
}