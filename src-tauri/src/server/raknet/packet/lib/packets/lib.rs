use serde::{ Deserialize, Serialize };
use num_derive::{ FromPrimitive, ToPrimitive };
use crate::stream::Serializable;

use super::super::NetworkStream;

pub use num_traits::{ FromPrimitive, ToPrimitive };

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NetworkEnum {
    pub name: String,
    pub size: u8,
    pub network_id: u16
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NetworkArgument {
    pub argument_type: NetworkPropertyType,
    pub argument_enum_id: u16
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NetworkEvent {
    pub name: String,   
    pub network_id: u16,
    pub arguments: Vec<NetworkArgument>
}

#[derive(Deserialize, Serialize, Debug, FromPrimitive, ToPrimitive, Eq, PartialEq, Clone)]
#[repr(u8)]
pub enum NetworkPropertyType {
    Unsupported = 0,
    Legacy_Deprecated1 = 1,
    String_NeverDictionary = 2,
    ProtectedStringServerIndexString = 3,
    ProtectedStringLegacyBytecode_Deprecated = 4,
    ProtectedStringCurrentBytecode_Deprecated = 5,
    ProtectedStringSource = 6,
    Enum_VarInt = 7,
    BinaryString = 8,
    Bool = 9,
    Int = 0x0A,
    Float = 0x0B,
    Double = 0x0C,
    UDim = 0x0D,
    UDim2 = 0x0E,
    Ray = 0x0F,
    Faces = 0x10,
    Axes = 0x11,
    BrickColor = 0x12,
    Color3 = 0x13,
    Color3uint8 = 0x14,
    Vector2 = 0x15,
    Vector3_Fixed12Bytes = 0x16,
    Vector3_PartSizeEncoding_Deprecated = 0x17,
    Vector2int16 = 0x18,
    Vector3int16 = 0x19,
    CoordinateFrame_ExactEncoding = 0x1A,
    CoordinateFrame_GeneralEncoding = 0x1B,
    InstanceGuid = 0x1C,
    Tuple = 0x1D,
    ValueArray = 0x1E,
    ValueTable = 0x1F,
    ValueMap = 0x20,
    ContentId = 0x21,
    SystemAddress = 0x22,
    NumberSequence = 0x23,
    NumberSequenceKeypoint = 0x24,
    NumberRange = 0x25,
    ColorSequence = 0x26,
    ColorSequenceKeypoint = 0x27,
    Rect2d = 0x28,
    PhysicalProperties = 0x29,
    Region3 = 0x2A,
    Region3int16 = 0x2B,
    Int64 = 0x2C,
    PathWaypoint_Deprecated = 0x2D,
    SharedString = 0x2E,
    ProtectedStringBytecode = 0x2F,
    DateTime = 0x30,
    String_FixedDictionary = 0x31,
    OptionalCoordinateFrame_ExactEncoding = 0x32,
    OptionalCoordinateFrame_GeneralEncoding = 0x33,
    UniqueId = 0x34,
    PathWaypointWithLabel = 0x35,
    Font = 0x36,
    COUNT = 0x37,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NetworkProperty {
    pub name: String,
    pub network_id: u16,
    pub prop_type: NetworkPropertyType,
    pub prop_enum_id: Option<u16>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NetworkClass {
    pub name: String,
    pub network_id: u16,
    pub properties: Vec<NetworkProperty>,
    pub events: Vec<NetworkEvent>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ShortAddress {
    pub version: u8,
    pub address: [u8; 4],
    pub port: u16
}

impl Serializable<ShortAddress> for ShortAddress {

    fn read( stream: &mut NetworkStream ) -> ShortAddress {
        let version = stream.read_byte();
        let bytes = stream.read_bytes( 4 )
            .iter()
            .map( |d| d ^ 0xFF )
            .collect::<Vec<u8>>();

        let port = stream.read_le();
        let address: [u8; 4] = bytes.as_slice().try_into().unwrap();

        ShortAddress { 
            version, 
            address,
            port 
        }
    }

    fn write( &mut self, stream: &mut NetworkStream ) {
        stream.write_byte( self.version );
        stream.write_bytes( self.address.to_vec() );
        stream.write_le( self.port );
    }

}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NetworkSchema {
    pub enums: Vec<NetworkEnum>,
    pub classes: Vec<NetworkClass>,
    pub content_prefixes: Vec<String>,
    pub optimized_strings: Vec<String>
}