use super::{ Patch, IDAPat, Scanner, PatchType, ReplacementPatch, OffsetPatch };
use lazy_static::lazy_static;
use std::rc::Rc;
use std::cell::RefCell;

pub struct InternalStudioPatch;

lazy_static! {
    static ref PATCHES: Vec<PatchType> = vec![
        ReplacementPatch::new(
            IDAPat::new( "83 CB 04 89 5E 64" ),
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

impl InternalStudioPatch {

    pub fn new() -> Patch {
        Patch {
            name: "internal-studio".into(),
            patch: InternalStudioPatch::patch
        }
    }

    pub fn patch( data: Rc<RefCell<Vec<u8>>> ) -> Result<(), String> {
        let scanner = Scanner::new( data.clone() );

        for patch in PATCHES.iter() {
            patch.patch( 
                &scanner,
                data.clone()
            )?;
        }

        Ok(())
    }

}