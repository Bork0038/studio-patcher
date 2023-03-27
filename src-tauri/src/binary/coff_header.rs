#[derive(Clone, Copy)]
#[repr(C)]
pub struct COFFHeader {
    pub signature: u32,
    pub machine: u16,
    pub number_of_sections: u16,
    pub time_data_stamp: u32,
    pub ptr_symbol_table: u32,
    pub num_symbol_table: u32,
    pub size_optional_header: u16,
    pub characteristics: u16
}