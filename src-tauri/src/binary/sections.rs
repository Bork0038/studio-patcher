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

pub struct Section {
    pub header: SectionHeader,
    pub data: Vec<u8>
}