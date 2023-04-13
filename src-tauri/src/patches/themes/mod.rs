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

    static ref THEME_LOAD_PAT: &'static str = "BA 2B 00 00 00 48 8D 0D ?? ?? ?? ?? FF 15 ?? ?? ?? ?? 48 89 85 88 00 00 00 BA 2A 00 00 00 48 8D 0D ?? ?? ?? ?? FF 15 ?? ?? ?? ?? 48 89 85 90 00 00 00 BA 02 00 00 00 48 8D 8D 50 01 00 00 E8 ?? ?? ?? ?? 4C 8B E0 48 89 85 50 01 00 00 48 83 C0 10 48 89 85 60 01 00 00 48 8D 9D 88 00 00 00 49 8B F4 0F 1F 40 00 48 8B D3 48 8B CE FF 15 ?? ?? ?? ?? 48 83 C6 08 48 83 C3 08 48 8D 85 98 00 00 00 48 3B D8 75 E0";
    static ref THEME_LOAD_PATCH: PatchType = ReplacementPatch::new(
        IDAPat::new( THEME_LOAD_PAT.clone() ),
        ".text",
        "RBXQT::Theme::Theme",
        vec![
            OffsetPatch::new(
                std::iter::repeat( 0x90 )
                    .take(
                        THEME_LOAD_PAT.split_whitespace().count()  
                    )
                    .collect(),
                0
            )
        ] 
    );
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

        let ( themes_section, text_section, theme_load_addr ) = {
            let mut bin = binary.borrow_mut();
            bin.add_section( new_section );
            bin.reload()?;
    
            let themes_section = bin
                .get_section_by_name( ".themes" )
                .map_or(
                    Err("Failed to find .themes section"), 
                    | section | Ok(section)
                )?;
    
            let text_section = bin
                .get_section_by_name( ".text" )
                .map_or(
                    Err("Failed to find .text section"), 
                    | section | Ok(section)
                )?;   

            let theme_load_addr = bin
                .scan(
                    &IDAPat::new( THEME_LOAD_PAT.clone() ),
                    Some( ".text" )
                )
                .map_or(
                    Err("Failed to find RBXQT::Theme::Theme"),
                    | addr | Ok( addr )
                )?;
    
            ( themes_section, text_section, theme_load_addr )
        };

        let themes_rva = themes_section.header.virtual_address.get(LittleEndian);
        let text_rva = text_section.header.virtual_address.get(LittleEndian);

        THEME_LOAD_PATCH.patch( binary.clone() )?;

        let offset = themes_rva - ( text_rva + theme_load_addr as u32);
        println!("Offset: {:02X}", offset);

        Ok(())
    }

}