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
            &IDAPat::new( "83 CB 04 89 5E 64" )
        ).map_or(
            Err( "Failed to find ::hasInternalPermission"),
            | addr | Ok(addr)
        )?;

        patch_segment(
            data, 
            vec![ 
                0x83, 0xCB, 0x05, 
            ],
            addr
        );

        Ok(())
    }

}