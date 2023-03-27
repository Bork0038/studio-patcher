#[derive(Clone, Copy)]
#[repr(C)]
pub struct COFFFields {
    pub magic: u16,
    pub major_linker_ver: u8,
    pub minor_linker_ver: u8,
    pub size_of_code: u32,
    pub size_of_initialized_data: u32,
    pub size_of_uninitialized_data: u32,
    pub address_entry_point: u32,
    pub base_of_code: u32
}