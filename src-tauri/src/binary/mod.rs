mod section;
pub use section::Section;

use std::error::Error;
use std::rc::Rc;
use std::cell::RefCell;
use object::{
    pe::{
        ImageNtHeaders64, 
        ImageDosHeader,
    },
    read::pe::{
        ImageNtHeaders,
        ImageOptionalHeader,
    },
    read::{
        Object as ReadObject, 
        pe::RichHeaderInfo
    },
    LittleEndian,
    File,
    write::{Object as WriteObject, pe::NtHeaders},
    write::pe::Writer
};

pub struct Binary;

impl Binary {

    pub fn add_section( data: Rc<RefCell<Vec<u8>>>, section: Section ) -> Result<(Vec<u8>, u32),  Box<dyn Error>> {
        let mut data = data.borrow_mut();
        let data: &[u8] = data.as_mut_slice();
      
        let dos_header = ImageDosHeader::parse( data )?;
        let mut offset = dos_header.nt_headers_offset().into();
        let rich_header = RichHeaderInfo::parse( data, offset );
        
        let (nt_headers, data_directories) = ImageNtHeaders64::parse( data, &mut offset )?;
        let file_header = nt_headers.file_header();
        let opt_header = nt_headers.optional_header();
        let sections = file_header.sections( data, offset )?;

        let mut out = Vec::new();
        let mut writer = Writer::new(
            nt_headers.is_type_64(),
            opt_header.section_alignment(),
            opt_header.file_alignment(),
            &mut out
        );

        writer.reserve_dos_header_and_stub();
        if let Some(rich_header) = rich_header { 
            writer.reserve( rich_header.length as u32 + 8,  4);
        }
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
        for (index, section) in sections.iter().enumerate() {
            let range = writer.reserve_section(
                section.name,
                section.characteristics.get(LittleEndian),
                section.virtual_size.get(LittleEndian),
                section.size_of_raw_data.get(LittleEndian)
                
            );

            reserved_sections.push((
                range.file_offset,
                section.pe_data( data )?
            ));
        }
        
        let range = writer.reserve_section(
            section.header.name,
            section.header.characteristics,
            section.header.virtual_size,
            section.header.size_of_raw_data
            
        );

        reserved_sections.push((
            range.file_offset,
            section.data.as_ref()
        ));

        writer.write_dos_header_and_stub()?;
        if let Some(rich_header) = rich_header {
            writer.write_align( 4 );
            writer.write( &data[rich_header.offset..][..rich_header.length] );
        }

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
            writer.write_section( index, data );
        }
        writer.write_reloc_section();
        
        Ok( 
            ( out, range.file_offset )
        )
    }

}