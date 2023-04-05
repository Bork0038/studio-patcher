mod lib;

use super::{ 
    Patch, 
    IDAPat,
    Scanner,
    PatchType, 
    ReplacementPatch, 
    OffsetPatch, 
    super::binary::{
        Binary,
        Section
    }
};
use std::fs::read_dir;
use lazy_static::lazy_static;
use lib::QArrayData;
use std::rc::Rc;
use std::cell::RefCell;
use object::{
    File,
    read::{
        Object, 
        ObjectSection
    }
};

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

    pub fn patch( data: Rc<RefCell<Vec<u8>>> ) -> Result<(), Box<dyn std::error::Error>> {
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

        let mut binary = Binary::new( data )?;
        binary.add_section( new_section );

        let section = binary.get_section_by_name(".themes").unwrap();
        println!("{}", Binary::get_section_name( &section.0 ));
        // let (mut new_data, offset) = Binary::add_section( 
        //     data.clone(), 
        //     new_section
        // )?;
        
        // let mut data = data.borrow_mut();
        // data.clear();   
        // data.append( &mut new_data );
    
        // let address = File::parse( &**data )?
        //     .section_by_name(".themes")
        //     .map_or(
        //         Err("Failed to find theme section"),
        //         | a | Ok(a.address())
        //     )?;

       
        // println!("map_address: {:02X}", address as usize + section.len() );
        // for offset in offset_map {
        //     println!("{:02X}", address as u64 + offset as u64);
        //     section.append(
        //         &mut u64::to_le_bytes( address as u64 + offset as u64 ).to_vec()
        //     );
        // }

        // let end_address = address as usize + section.len();
        // let mut end_segment = Vec::from([ 255; 8 ]);
        // section.append( &mut end_segment );
        
        // for i in 0..section.len() {
        //     data[ offset as usize + i ] = section[ i ];
        // }

        // println!("end_section: {:02X}", end_address);

        
        // let scanner = Scanner::new( data );

        // for patch in PATCHES.iter() {
        //     patch.patch( 
        //         &scanner,
        //         data
        //     )?;
        // }

        Ok(())
    }

}