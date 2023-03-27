#[derive(Clone, Copy)]
#[repr(C)]
pub struct DataDirectory {
    pub virtual_address: u32,
    pub size: u32
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct DataDirectories {
    pub export_table: DataDirectory,
    pub import_table: DataDirectory,
    pub resource_table: DataDirectory,
    pub exception_table: DataDirectory,
    pub certificate_table: DataDirectory,
    pub base_relocation_table: DataDirectory,
    pub debug: DataDirectory,
    pub architecture_data: DataDirectory,
    pub global_ptr: u32,
    
    // reserved bytes
    res_1: u32,

    pub tls_table: DataDirectory,
    pub load_config_table: DataDirectory,
    pub bound_import: DataDirectory,
    pub iat: DataDirectory,
    pub delay_import_desc: DataDirectory,
    pub clr_runtime_header: DataDirectory,
    
    // reserved_bytes
    res_2: u64
}

