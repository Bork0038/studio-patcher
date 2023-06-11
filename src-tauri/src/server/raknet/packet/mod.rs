mod lib;

use std::fmt;
use super::NetworkStream;
use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Packet {
    ID_FCAR_REQUIRES_PUBLIC_KEY { id: u16 },
    ID_FCAR_REQUIRES_SECURITY { id: u16 },
    ID_FCAR_PUBLIC_KEY_MISMATCH { id: u16 },
    ID_CONNECTION_REQUEST_ACCEPTED { id: u16 },

    ID_FCAR_CONNECTION_ATTEMPT_FAILED { id: u16 },
    ID_FCAR_ALREADY_CONNECTED { id: u16 },
    ID_FCAR_NO_FREE_INCOMING_CONNECTIONS { id: u16 },
    ID_FCAR_HASH_MISMATCH { id: u16 },
    ID_FCAR_SECURITYKEY_MISMATCH { id: u16 },
    ID_FCAR_CONNECTION_BANNED { id: u16 },
    ID_FCAR_INVALID_PASSWORD { id: u16 },
    ID_FCAR_INCOMPATIBLE_PROTOCOL { id: u16 },
    ID_FCAR_IP_RECENTLY_CONNECTED { id: u16 },

    ID_TIMESTAMP {
        id: u16,
        last_time: u64,
        flag: u8
    },

    ID_SET_GLOBALS { // not done -- pushIncomingPackets 
        id: u16,
        streaming_enabled: bool,
        filtering_enabled: bool,
        third_party_asset_allowed: bool,
        third_party_purchase_allowed: bool,
        third_party_teleport_allowed: bool,
        peer_id: u32,
    },
    
    ID_MARKER {
        id: u16,
        marker_id: i32
    },

    ID_PHYSICS { // no clue where this comes from
        id: u16
    },
    
    ID_TOUCHES {  // not done -- pushIncomingPackets 
        id: u16,
    },

    ID_CHAT_ALL {
        id: u16,
        guid_index_1: i32,
        guid_index_2: i32
    },

    ID_SFFLAG_SYNC {
        id: u16,
        fflags: Vec<(String, String)>
    },

    ID_NEW_SCHEMA {
        id: u16
    },

    ID_REPLIC_PING {
        id: u16,
        type_and_version: u8,
        time: u64,
        first_serialize_out_of_time: u32,
        peer_ping_arrival_local_read_time: Option<u32>,
        peer_reply_queue_time: Option<u32>,
        peer_reply_serialize_time: Option<u32>,
        peer_last_known_bcs_queue_time: Option<u32>,
        peer_last_known_recv_to_pop_time: Option<u32>,
        peer_last_known_pop_to_read_time: Option<u32>,
        render_fps: Option<f32>,
        physics_fps: Option<f32>,
        heartbeat_fps: Option<f32>,
        send_stats: u32,
        extra_stats: u32
    },

    ID_REPLIC_PING_BACK {
        id: u16,
        type_and_version: u8,
        time: u64,
        first_serialize_out_of_time: u32,
        peer_ping_arrival_local_read_time: Option<u32>,
        peer_reply_queue_time: Option<u32>,
        peer_reply_serialize_time: Option<u32>,
        peer_last_known_bcs_queue_time: Option<u32>,
        peer_last_known_recv_to_pop_time: Option<u32>,
        peer_last_known_pop_to_read_time: Option<u32>,
        render_fps: Option<f32>,
        physics_fps: Option<f32>,
        heartbeat_fps: Option<f32>,
        send_stats: u32,
        extra_stats: u32
    },

    ID_REPLIC_TAG {
        id: u16,
        tag_id: lib::packets::packet_83_10::TagItemType
    },

    ID_REPLIC_MARKER {
        id: u16,
        marker_id: i32
    }
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}