#[derive(Clone, Copy)]
#[repr(C)]
pub struct WinFields {
    pub image_base: u64,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub major_os_ver: u16,
    pub minor_os_ver: u16,
    pub major_img_ver: u16,
    pub minor_img_ver: u16,
    pub major_subsys_ver: u16,
    pub minor_subsys_ver: u16,
    pub win32_ver: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub checksum: u32,
    pub subsystem: u16,
    pub dll_characteristics: u16,
    pub size_of_stack_reserve: u64,
    pub size_of_stack_commit: u64,
    pub size_of_heap_reserve: u64,
    pub size_of_heap_commit: u64,
    pub loader_flags: u32,
    pub num_of_rva_sizes: u32
}