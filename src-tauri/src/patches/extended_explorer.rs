use super::{ Patch, IDAPat, Scanner, ReplacementPatch, PatchType, OffsetPatch };
use lazy_static::lazy_static;
use std::rc::Rc;
use std::cell::RefCell;

pub struct ExtendedExplorerPatch;

lazy_static! {
    static ref PATCHES: Vec<PatchType> = vec![
       
    ];
}

impl ExtendedExplorerPatch {

    pub fn new() -> Patch {
        Patch {
            name: "themes".into(),
            patch: ExtendedExplorerPatch::patch
        }
    }

    pub fn patch( data: Rc<RefCell<Vec<u8>>> ) -> Result<(), Box<dyn std::error::Error>> {
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