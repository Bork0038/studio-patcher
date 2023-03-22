use super::{ Patch, IDAPat, patch_segment, Scanner };
use std::{collections::HashMap, borrow::BorrowMut};

pub struct ExtendedExplorerPatch {}

impl ExtendedExplorerPatch {

    pub fn new() -> Patch {
        Patch {
            name: String::from( "extended-explorer" ),
            patch: ExtendedExplorerPatch::patch
        }
    }

    pub fn patch( data: &mut Box<Vec<u8>> ) -> Result<(), String> {
        let scanner = Scanner::new( &data );
        {
            let addr = match scanner.scan(
                &IDAPat::new( "48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 48 89 7C 24 20 41 56 48 83 EC 40 4C 8B F1 48 8B 59 60 48 85 DB 74 08 F0 FF 43 08 48 8B 59 60" )
            ) {
                Some(addr) => addr,
                None => return Err(
                    String::from( "Failed to find PropertyItem::currentlyHidden" )
                )
            };

            patch_segment(
                &mut data.clone(), 
                vec![ 
                    0xE9, 0x01, 0x01, 0x00, 0x00, // jmp loc_7FF6A5760A40
                    0x90,                         // nop
                    0x90                          // nop
                ],
                addr + 26
            );
        }
        {
            let addr = match scanner.scan(
                &IDAPat::new( "48 89 5C 24 08 48 89 74 24 10 57 48 83 EC 40 48 8B F2 48 8B CA E8 ?? ?? ?? ?? 84 C0 0F 84 D7 00 00 00 48 8B 0E 48 8B 01 FF 50" )
            ) {
                Some(addr) => addr,
                None => return Err(
                    String::from( "Failed to find descriptorVisibleInWidget" )
                )
            };

            patch_segment(
                data, 
                vec![ 
                    0xE9, 0xE5, 0x00, 0x00, 0x00, // jmp loc_7FF779CB34D9
                    0x90,                         // nop 
                ],
                addr + 15
            );

            patch_segment(
                data, 
                vec![ 
                    0xB0, 0x01 // mov al, 1
                ],
                addr + 249
            );
        }

        Ok(())
    }
 
}