mod coff_fields;
mod win_fields;
mod data_directories;

use coff_fields::COFFFields;
use win_fields::WinFields;
use data_directories::DataDirectories;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct OptionalHeader {
    pub coff_fields: COFFFields,
    pub win_fields: WinFields,
    pub data_directories: DataDirectories
}