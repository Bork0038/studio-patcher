use super::Patch;
use crate::binary::{ IDAPat, Binary };
use std::rc::Rc;
use std::cell::RefCell;
use std::error::Error;
use lazy_static::lazy_static;

lazy_static! {
    
}

pub struct InternalStudioPatch;

impl InternalStudioPatch {

    pub fn new() -> Patch {
        Patch {
            name: "internal-studio".into(),
            patch: InternalStudioPatch::patch
        }
    }

    pub fn patch( binary: Rc<RefCell<Binary>> ) -> Result<(), Box<dyn Error>> {


        Ok(())
    }

}