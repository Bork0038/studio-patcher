use super::{ Patch, PatchType, ReplacementPatch, OffsetPatch};
use crate::binary::{ IDAPat, Binary };
use std::rc::Rc;
use std::cell::RefCell;
use std::error::Error;
use lazy_static::lazy_static;

lazy_static! {
    static ref PATCHES: Vec<PatchType> = vec![
        ReplacementPatch::new(
            IDAPat::new( "83 CB 04 89 5E 64" ),
            ".text",
            "RBX::hasInternalPermission",
            vec![
                OffsetPatch::new(
                    vec![ 
                        0x83, 0xCB, 0x05, 
                    ],
                    0
                )
            ]
        )
    ];
}

pub struct InternalStudioPatch;

impl InternalStudioPatch {

    pub fn new() -> Patch {
        Patch {
            name: "internal-studio".into(),
            patch: InternalStudioPatch::patch
        }
    }

    pub fn patch( binary: Rc<RefCell<Binary>>, file_path: String ) -> Result<(), Box<dyn Error>> {
        for patch in PATCHES.iter() {
            patch.patch( binary.clone() )?;
        }

        Ok(())
    }

}