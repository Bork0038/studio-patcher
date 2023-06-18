use serde::{ Deserialize, Serialize };
use num_derive::{ FromPrimitive, ToPrimitive };

pub use num_traits::{ FromPrimitive, ToPrimitive };

#[derive(Deserialize, Serialize)]
pub struct NetworkEnum {
    pub name: String,
    pub size: u8,
    pub network_id: u16
}

#[derive(Deserialize, Serialize, Debug, FromPrimitive, ToPrimitive, Eq, PartialEq)]
#[repr(u8)]
pub enum NetworkPropertyType {
    Void = 0,
    Bool = 1,
    Int = 2,
    Int64 = 3,
    Float = 4,
    Double = 5,
    String = 6,
    ProtectedString = 7,
    Instance = 8,
    Instances = 9,
    Ray = 0x0A,
    Vector2 = 0x0B,
    Vector3 = 0x0C,
    Vector2Int16 = 0x0D,
    Vector3Int16 = 0x0E,
    Rect2d = 0x0F,
    CoordinateFrame = 0x10,
    Color3 = 0x11,
    Color3uint8 = 0x12,
    UDim = 0x13,
    UDim2 = 0x14,
    Faces = 0x15,
    Axes = 0x16,
    Region3 = 0x17,
    Region3Int16 = 0x18,
    CellId = 0x19,
    GuidData = 0x1A,
    PhysicalProperties = 0x1B,
    BrickColor = 0x1C,
    SystemAddress = 0x1D,
    BinaryString = 0x1E,
    Surface = 0x1F,
    Enum = 0x20,
    Property = 0x21,
    Tuple = 0x22,
    ValueArray = 0x23,
    ValueTable = 0x24,
    ValueMap = 0x25,
    Variant = 0x26,
    GenericFunction = 0x27,
    WeakFunctionRef = 0x28,
    ColorSequence = 0x29,
    ColorSequenceKeypoint = 0x2A,
    NumberRange = 0x2B,
    NumberSequence = 0x2C,
    NumberSequenceKeypoint = 0x2D,
    InputObject = 0x2E,
    Connection = 0x2F,
    ContentId = 0x30,
    DescribedBase = 0x31,
    RefType = 0x32,
    QFont = 0x33,
    QDir = 0x34,
    EventInstance = 0x35,
    TweenInfo = 0x36,
    DockWidgetPluginGuiInfo = 0x37,
    PluginDrag = 0x38,
    Random = 0x39,
    PathWaypoint = 0x3A,
    FloatCurveKey = 0x3B,
    RotationCurveKey = 0x3C,
    SharedString = 0x3D,
    DateTime = 0x3E,
    RaycastParams = 0x3F,
    RaycastResult = 0x40,
    OverlapParams = 0x41,
    LazyTable = 0x42,
    DebugTable = 0x43,
    CatalogSearchParams = 0x44,
    OptionalCoordinateFrame = 0x45,
    CSGPropertyData = 0x46,
    UniqueId = 0x47,
    Font = 0x48,
    Blackboard = 0x49,
    Max = 0x4A
}

#[derive(Deserialize, Serialize)]
pub struct NetworkProperty {
    pub name: String,
    pub network_id: u16,
    pub prop_type: NetworkPropertyType,
    pub prop_enum_id: Option<u16>
}

#[derive(Deserialize, Serialize)]
pub struct NetworkClass {
    pub name: String,
    pub network_id: u16,
    pub properties: Vec<NetworkProperty>
}

