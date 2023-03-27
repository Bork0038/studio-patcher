pub const IMAGE_SCN_TYPE_NO_PAD: usize = 0x8;
pub const IMAGE_SCN_CNT_CODE: usize = 0x20;
pub const IMAGE_SCN_CNT_INITIALIZED_DATA: usize = 0x40;
pub const IMAGE_SCN_CNT_UNINITIALIZED_DATA: usize = 0x80;
pub const IMAGE_SCN_LNK_OTHER: usize = 0x100;
pub const IMAGE_SCN_LNK_INFO: usize = 0x200;
pub const IMAGE_SCN_LNK_REMOVE: usize = 0x800;
pub const IMAGE_SCN_LNK_COMDAT: usize = 0x1000;
pub const IMAGE_SCN_GPREL: usize = 0x8000;
pub const IMAGE_SCN_MEM_PURGEABLE: usize = 0x20000; // can't use enum because of these two
pub const IMAGE_SCN_MEM_16BIT: usize = 0x20000;
pub const IMAGE_SCN_MEM_LOCKED: usize = 0x40000;
pub const IMAGE_SCN_MEM_PRELOAD: usize = 0x80000;
pub const IMAGE_SCN_ALIGN_1BYTES: usize = 0x100000;
pub const IMAGE_SCN_ALIGN_2BYTES: usize = 0x200000;
pub const IMAGE_SCN_ALIGN_4BYTES: usize = 0x300000;
pub const IMAGE_SCN_ALIGN_8BYTES: usize = 0x400000;
pub const IMAGE_SCN_ALIGN_16BYTES: usize = 0x500000;
pub const IMAGE_SCN_ALIGN_32BYTES: usize = 0x600000;
pub const IMAGE_SCN_ALIGN_64BYTES: usize = 0x700000;
pub const IMAGE_SCN_ALIGN_128BYTES: usize = 0x800000;
pub const IMAGE_SCN_ALIGN_256BYTES: usize = 0x900000;
pub const IMAGE_SCN_ALIGN_512BYTES: usize = 0xA00000;
pub const IMAGE_SCN_ALIGN_1024BYTES: usize = 0xB00000;
pub const IMAGE_SCN_ALIGN_2048BYTES: usize = 0xC00000;
pub const IMAGE_SCN_ALIGN_4096BYTES: usize = 0xD00000;
pub const IMAGE_SCN_ALIGN_8192BYTES: usize = 0xE00000;
pub const IMAGE_SCN_LNK_NRELOC_OVFL: usize = 0x1000000;
pub const IMAGE_SCN_MEM_DISCARDABLE: usize = 0x2000000;
pub const IMAGE_SCN_MEM_NOT_CACHED: usize = 0x4000000;
pub const IMAGE_SCN_MEM_NOT_PAGED: usize = 0x8000000;
pub const IMAGE_SCN_MEM_SHARED: usize = 0x10000000;
pub const IMAGE_SCN_MEM_EXECUTE: usize = 0x20000000;
pub const IMAGE_SCN_MEM_READ: usize = 0x40000000;
pub const IMAGE_SCN_MEM_WRITE: usize = 0x80000000;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SectionHeader {
    pub name: [u8; 8],
    pub virtual_size: u32,
    pub virtual_address: u32,
    pub size_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
    pub pointer_to_relocations: u32,
    pub pointer_to_line_numbers: u32,
    pub number_of_relocations: u16,
    pub number_of_line_numbers: u16,
    pub characteristics: u32
}

pub struct Section {
    pub header: SectionHeader,
    pub data: Vec<u8>
}

impl Section {

    pub fn new<S: Into<String>, D: AsRef<[u8]>>( name: S, data: D ) -> Self {
        // theres probably a better way of doing this
        let name: [u8; 8] = {
            let mut name: Vec<char> = name
                .into()
                .chars()
                .collect();

            let len = name.len();
            if len > 8 {
                name[ 0..len ]
                    .iter()
                    .map( |c| *c as u8 )
                    .collect::<Vec<u8>>()
                    .as_slice()
                    .try_into()
                    .unwrap()
            } else {
                let mut padding = std::iter::repeat( 0 as char )
                    .take( 8 - len )
                    .collect::<Vec<char>>();
                name.append( &mut padding );

                name
                    .iter()
                    .map( |c| *c as u8 )
                    .collect::<Vec<u8>>()
                    .as_slice()
                    .try_into()
                    .unwrap()
            }
        };

        Section {
            data: data.as_ref().to_vec(),
            header: SectionHeader { 
                name,	
                virtual_size: 0,	
                virtual_address: 0,	
                size_of_raw_data: 0,	
                pointer_to_raw_data: 0,	
                pointer_to_relocations: 0,	
                pointer_to_line_numbers: 0,	
                number_of_relocations: 0,	
                number_of_line_numbers: 0,	
                characteristics: (IMAGE_SCN_MEM_WRITE | IMAGE_SCN_MEM_READ | IMAGE_SCN_MEM_EXECUTE | IMAGE_SCN_CNT_CODE | IMAGE_SCN_CNT_INITIALIZED_DATA) as u32
            }
        }
    }

}


impl SectionHeader {

    pub fn get_name( &self ) -> String {
        let name = self.name.to_vec();

        let iter = name
            .iter()
            .filter( |c| **c != 0 )
            .map( |x| *x )
            .collect();

        String::from_utf8( iter ).unwrap()
    }

}
