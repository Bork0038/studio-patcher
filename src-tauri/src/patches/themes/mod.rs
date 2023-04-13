mod lib;
use lib::*;

use super::{ Patch, PatchType, ReplacementPatch, OffsetPatch};
use crate::binary::{ IDAPat, Binary, Section };
use lazy_static::lazy_static;
use std::rc::Rc;
use std::cell::RefCell;
use std::error::Error;
use std::fs::read_dir;
use object::LittleEndian;

pub struct ThemesPatch;

lazy_static! {
    static ref PATCHES: Vec<PatchType> = vec![
        
    ];
}

impl ThemesPatch {

    pub fn new() -> Patch {
        Patch {
            name: "themes".into(),
            patch: ThemesPatch::patch
        }
    }

    pub fn patch( binary: Rc<RefCell<Binary>> ) -> Result<(), Box<dyn Error>> {
        let mut themes = vec![
            String::from( ":/Platform/Base/QtUI/themes/DarkTheme.json" ),
            String::from( ":/Platform/Base/QtUI/themes/LightTheme.json" )
        ];

        for file in read_dir("../themes")? {
            let file = file?;
            let metadata = file.metadata()?;

            if metadata.is_file() {
                if let Some(path) = file.path().canonicalize()?.to_str() {
                    themes.push( 
                        String::from( path )
                    );
                }
            }
        }

        let mut section = Vec::new();
        let mut offset_map = Vec::new();
        unsafe {
            for theme in themes {
                offset_map.push( section.len() );
                section.append(
                    &mut QArrayData::serialize( &theme )
                );
            }
        }

        let section_size = 
            section.len() 
            + offset_map.len() * 8 
            + 8;

        let new_section = Section::new(
            ".themes",
            std::iter::repeat( 0x00 ).take( section_size ).collect::<Vec<u8>>() 
        );

        let mut binary = binary.borrow_mut();
        binary.add_section( new_section );
        binary.reload()?;

        let themes_section = binary
            .get_section_by_name( ".themes" )
            .map_or(
                Err("Failed to find .themes section"), 
                | section | Ok(section)
            )?;

        let text_section = binary
            .get_section_by_name( ".text" )
            .map_or(
                Err("Failed to find .text section"), 
                | section | Ok(section)
            )?;

        let themes_rva = themes_section.header.virtual_address.get(LittleEndian);
        let text_rva = text_section.header.virtual_address.get(LittleEndian);

        println!("{:02X}", themes_rva - text_rva);

        Ok(())
    }

}