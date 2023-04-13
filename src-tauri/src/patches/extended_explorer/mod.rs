use super::{ Patch, PatchType, ReplacementPatch, OffsetPatch};
use crate::binary::{ IDAPat, Binary };
use std::rc::Rc;
use std::cell::RefCell;
use std::error::Error;
use lazy_static::lazy_static;

lazy_static! {
    static ref PATCHES: Vec<PatchType> = vec![
        ReplacementPatch::new(
            IDAPat::new( "48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 48 89 7C 24 20 41 56 48 83 EC 40 4C 8B F1 48 8B 59 60 48 85 DB 74 08 F0 FF 43 08 48 8B 59 60" ),
            ".text",
            "PropertyItem::currentlyHidden",
            vec![
                OffsetPatch::new(
                    vec![ 
                        0xE9, 0x01, 0x01, 0x00, 0x00, // jmp loc_7FF6A5760A40
                        0x90,                         // nop
                        0x90                          // nop
                    ],
                    26
                )
            ]
        ),
        ReplacementPatch::new(
            IDAPat::new( "48 89 5C 24 08 48 89 74 24 10 57 48 83 EC 40 48 8B F2 48 8B CA E8 ?? ?? ?? ?? 84 C0 0F 84 D7 00 00 00 48 8B 0E 48 8B 01 FF 50" ),
            ".text",
            "RobloxPropertiesTreeWidget::descriptorVisibleInWidget",
            vec![
                OffsetPatch::new(
                    vec![ 
                        0xE9, 0xE5, 0x00, 0x00, 0x00, // jmp loc_7FF779CB34D9
                        0x90,                         // nop 
                    ],
                    15
                ),
                OffsetPatch::new(
                    vec![ 
                        0xB0, 0x01 // mov al, 1
                    ],
                    249
                )
            ]
        )
    ];
}

pub struct ExtendedExplorerPatch ;

impl ExtendedExplorerPatch  {

    pub fn new() -> Patch {
        Patch {
            name: "extended-explorer".into(),
            patch: ExtendedExplorerPatch ::patch
        }
    }

    pub fn patch( binary: Rc<RefCell<Binary>> ) -> Result<(), Box<dyn Error>> {
        for patch in PATCHES.iter() {
            patch.patch( binary.clone() )?;
        }

        Ok(())
    }

}
