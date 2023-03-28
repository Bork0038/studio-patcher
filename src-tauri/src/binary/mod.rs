mod dos_header;
mod coff_header;
mod optional_header;
mod sections;

pub use dos_header::DOSHeader;
pub use coff_header::COFFHeader;
pub use optional_header::OptionalHeader;
pub use sections::{ Section, SectionHeader };

use std::mem::transmute;
use std::io::Write;
use positioned_io::WriteAt;
use num_traits::NumCast;

pub fn align_to<A, B, C>( value: A, alignment: B ) -> C 
where A: NumCast, B: NumCast, C: NumCast {
    let value = <usize as NumCast>::from( value ).unwrap();
    let alignment = <usize as NumCast>::from( alignment ).unwrap();

    let r = value % alignment;
    let res = if r != 0 { value + alignment - r  } else { value };

    <C as NumCast>::from( res ).unwrap()
}

const SIZE_NT_HEADERS: usize = 264;
const SIZE_SECTION_HEADER: usize = 40;


pub struct Binary {
    pub dos_header: DOSHeader,
    pub dos_stub: Vec<u8>,
    pub coff_header: COFFHeader,
    pub optional_header: OptionalHeader,
    pub sections: Vec<Section>
}

impl Binary {

    pub unsafe fn new<D: AsRef<[u8]>>( data: D ) -> Self {
        let data = data.as_ref();

        let dos_header = {
            let dos_slice: &[u8; 64] = &data[ 0..64]
                .try_into()
                .unwrap();

            transmute::<[u8; 64], DOSHeader>( *dos_slice )
        };

        let lfanew = dos_header.e_lfanew as usize;
        let dos_stub = Vec::from( &data[ 64 .. lfanew ]);
        
        let coff_header = {
            let coff_slice: &[u8; 24] = &data[ lfanew.. lfanew + 24 ]
                .try_into()
                .unwrap();

            transmute::<[u8; 24], COFFHeader>( *coff_slice )
        };

        let optional_header = {
            let optional_header_slice: &[u8; 240] = &data[ lfanew + 24..lfanew + 264 ]
                .try_into()
                .unwrap();

            transmute::<[u8; 240], OptionalHeader>( *optional_header_slice )
        };

        let mut section_start = lfanew + SIZE_NT_HEADERS; 
        let mut section_table = Vec::new();
        for _ in 0..coff_header.number_of_sections {
            let section_header = {
                let section_header_slice: &[u8; SIZE_SECTION_HEADER] = &data[ section_start..section_start + SIZE_SECTION_HEADER ]
                    .try_into()
                    .unwrap();

                transmute::<[u8; SIZE_SECTION_HEADER], SectionHeader>( *section_header_slice )
            };

            section_table.push( section_header );
            section_start += SIZE_SECTION_HEADER;
        }

        let mut section_data = Vec::new();
        for section in section_table {
            let start = section.pointer_to_raw_data as usize;
            let size  = section.size_of_raw_data as usize; 
            
            if size == 0 || start == 0 {
                section_data.push(
                    Section {
                        header: section,
                        data: Vec::new()
                    }
                );

                continue
            }

            let section_slice = &data[ start .. start + size ];
            section_data.push(
                Section {
                    header: section,
                    data: section_slice.to_vec()
                }
            );
        }

        Binary { 
            dos_header,
            dos_stub,
            coff_header,
            optional_header,
            sections: section_data
        }
    }

    pub fn add_section( &mut self, mut section: Section ) {
        let file_alignment = self.optional_header.win_fields.file_alignment as usize;
        let section_alignment = self.optional_header.win_fields.section_alignment as usize;

        self.coff_header.number_of_sections += 1;

        let last_section = self.sections
            .last()
            .unwrap();
        
        let rva = align_to(
            last_section.header.virtual_address + last_section.header.virtual_size, 
            section_alignment
        );

        let section_size = section.data.len() as u32;

        self.optional_header.coff_fields.size_of_initialized_data += section_size as u32;
        self.optional_header.win_fields.size_of_headers = align_to(
            self.dos_header.e_lfanew + SIZE_NT_HEADERS as u32 +
            self.coff_header.number_of_sections as u32 * SIZE_SECTION_HEADER as u32, 
            file_alignment
        );
        self.optional_header.win_fields.size_of_image = align_to( 
            rva + section_size,
            section_alignment
        );

        section.header.size_of_raw_data = section_size;
        section.header.virtual_size = section_size;

        section.header.pointer_to_raw_data = align_to(
            last_section.header.pointer_to_raw_data + last_section.header.size_of_raw_data,
            file_alignment
        );
        section.header.virtual_address = rva;

        self.sections.push( section );
    }

    pub unsafe fn compile( &mut self ) -> Box<Vec<u8>> {
        let file_alignment = self.optional_header.win_fields.file_alignment as usize;
        let mut out = Box::new( Vec::new() );

        {
            let dos_header_data = transmute::<DOSHeader, [u8; 64]>( self.dos_header );

            out
                .write( &dos_header_data )
                .unwrap();
        }

        out
            .write( &self.dos_stub )
            .unwrap();

        {
            let coff_header_data = transmute::<COFFHeader, [u8; 24]>( self.coff_header );

            out
                .write( &coff_header_data )
                .unwrap();
        }

        {
            let optional_header_data = transmute::<OptionalHeader, [u8; 240]>( self.optional_header );

            out
                .write( &optional_header_data )
                .unwrap();
        }

        for section in &self.sections {
            let header_data = transmute::<SectionHeader, [u8; SIZE_SECTION_HEADER]>( section.header );

            out
                .write( &header_data )
                .unwrap();  
        }   
        
        let mut index: u64 = align_to( out.len(), file_alignment );
        for section in &self.sections {
            out
                .write_at( index as u64, &section.data )
                .unwrap();

            index += align_to::<u32, usize, u64>( section.header.size_of_raw_data, file_alignment );
        }

        out
    }
}
