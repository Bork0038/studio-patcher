use super::{ Patch, IDAPat, Scanner, patch_segment };
use std::collections::HashMap;

pub struct InternalStudioPatch {}

impl InternalStudioPatch {

    pub fn new() -> Patch {
        Patch {
            name: String::from( "internal-studio" ),
            patch: InternalStudioPatch::patch
        }
    }

    pub fn patch( data: &mut Box<Vec<u8>> ) -> Result<(), &str> {
        let scanner = Scanner::new( &data );

        let addr = scanner.scan(
            &IDAPat::new( "48 89 5C 24 08 57 48 83 EC 50 0F 10 01 48 8D 4C 24 30 49 8B D8 48 8B FA 0F 29 44 24 30 E8 ?? ?? ?? ?? 84 C0 0F 85 C5 00 00 00 0F 10 03 48 C7 44 24 28 08 00 00 00" )
        ).map_or(
            Err( "Failed to find ::hasInternalPermission"),
            | addr | Ok(addr)
        )?;

        patch_segment(
            data, 
            vec![ 
                0xE9, 0xE0, 0x00, 0x00, 0x00, 
                0x90, 
                0x90, 
                0x90 
            ],
            addr + 10
        );

        Ok(())
    }

}