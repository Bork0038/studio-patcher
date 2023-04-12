mod section;
mod scanner;

pub use section::Section;
pub use scanner::*;

use std::error::Error;
use object::{
    pe::{
        ImageNtHeaders64, 
        ImageDosHeader, 
        ImageDataDirectory, 
        ImageFileHeader,
        ImageOptionalHeader64,
    },
    read::pe::{
        ImageNtHeaders,
        ImageOptionalHeader,
    },
    LittleEndian,
    write::pe::{ 
        Writer,
        NtHeaders
    }, 
};

pub struct Binary {
    dos_header: ImageDosHeader,
    nt_headers: ImageNtHeaders64,
    data_directories: Vec<ImageDataDirectory>,
    file_header: ImageFileHeader,
    opt_header: ImageOptionalHeader64,
    sections: Vec<Section>
}

impl Binary {
    
    pub fn new<D: AsRef<[u8]>>( data: D ) -> Result<Self, Box<dyn Error>> {
        let data = data.as_ref();
      
        let dos_header = *ImageDosHeader::parse( data )?;
        let mut offset = dos_header.nt_headers_offset().into();

        let (nt_headers, data_directories) = ImageNtHeaders64::parse( data, &mut offset )?;
        let file_header = nt_headers.file_header();

        let mut sections = Vec::new();  
        for section in file_header.sections(data, offset)?.iter() {
            sections.push(
                Section {
                    header: *section,
                    data: section.pe_data( data )?.to_vec()
                }
            );
        }

        Ok( Binary {
            dos_header,
            nt_headers: *nt_headers,
            data_directories: data_directories.iter().map(|d| *d).collect(),
            file_header: *file_header,
            opt_header: *nt_headers.optional_header(),
            sections
        } )
    }

    pub fn get_section_by_name<S: Into<String>>( &mut self, name: S ) -> Option<&mut Section> {
        let name = name.into();
       
        for section in self.sections.iter_mut() {
            if section.get_name() == name {
                return Some( section );
            }
        }

        None
    }

    pub fn add_section( &mut self, section: Section ) {
        self.sections.push( section );
    }

    pub fn serialize( &mut self ) -> Result<Vec<u8>, Box<dyn Error>> {
        let nt_headers = self.nt_headers;
        let opt_header = self.opt_header;
        let data_directories = &self.data_directories;
        let file_header = self.file_header;

        let mut out = Vec::new();   
        let mut writer = Writer::new(
            nt_headers.is_type_64(),
            opt_header.section_alignment(),
            opt_header.file_alignment(),
            &mut out
        );

        writer.reserve_dos_header_and_stub();
        writer.reserve_nt_headers( data_directories.len() );

        for (index, directory) in data_directories.iter().enumerate() {
            writer.set_data_directory( 
                index, 
                directory.virtual_address.get(LittleEndian), 
                directory.size.get(LittleEndian)
            );
        }   
        
        writer.reserve_section_headers( file_header.number_of_sections.get(LittleEndian) + 1 );
        
        let mut reserved_sections = Vec::new();
        for section in self.sections.iter() {
            let header = section.header;

            let range = writer.reserve_section(
                header.name,
                header.characteristics.get(LittleEndian),
                header.virtual_size.get(LittleEndian),
                header.size_of_raw_data.get(LittleEndian)
                
            );

            reserved_sections.push((
                range.file_offset,
                section.data.clone()
            ));
        }

        writer.write_dos_header_and_stub()?;
        writer.write_nt_headers(NtHeaders {
            machine: file_header.machine.get(LittleEndian),
            time_date_stamp: file_header.time_date_stamp.get(LittleEndian),
            characteristics: file_header.characteristics.get(LittleEndian),
            major_linker_version: opt_header.major_linker_version(),
            minor_linker_version: opt_header.minor_linker_version(),
            address_of_entry_point: opt_header.address_of_entry_point(),
            image_base: opt_header.image_base(),
            major_operating_system_version: opt_header.major_operating_system_version(),
            minor_operating_system_version: opt_header.minor_operating_system_version(),
            major_image_version: opt_header.major_image_version(),
            minor_image_version: opt_header.minor_image_version(),
            major_subsystem_version: opt_header.major_subsystem_version(),
            minor_subsystem_version: opt_header.minor_subsystem_version(),
            subsystem: opt_header.subsystem(),
            dll_characteristics: opt_header.dll_characteristics(),
            size_of_stack_reserve: opt_header.size_of_stack_reserve(),
            size_of_stack_commit: opt_header.size_of_stack_commit(),
            size_of_heap_reserve: opt_header.size_of_heap_reserve(),
            size_of_heap_commit: opt_header.size_of_heap_commit(),
        });

        writer.write_section_headers();
        for (index, data) in reserved_sections {
            writer.write_section( index, &*data );
        }
        writer.write_reloc_section();
        
        Ok( out )
    }

}