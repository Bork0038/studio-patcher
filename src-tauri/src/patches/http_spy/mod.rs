use super::{ Patch, PatchType, ReplacementPatch, OffsetPatch};
use crate::binary::{ IDAPat, Binary };
use std::rc::Rc;
use std::cell::RefCell;
use std::error::Error;

pub struct HttpSpyPatch;

impl HttpSpyPatch  {

    pub fn new() -> Patch {
        Patch {
            name: "http-spy".into(),
            patch: HttpSpyPatch ::patch
        }
    }

    pub fn patch( binary: Rc<RefCell<Binary>> ) -> Result<(), Box<dyn Error>> {
        let bin = binary.borrow_mut();
        
        Ok(())
    }

}
