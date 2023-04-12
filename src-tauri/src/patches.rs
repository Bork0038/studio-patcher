mod lib;
pub use lib::*;

mod internal_studio;

use internal_studio::InternalStudioPatch;

use crate::Binary;
use std::{ rc::Rc, cell::RefCell };
use lazy_static::lazy_static;

lazy_static! {
    static ref patch_list: Vec<Patch> = vec![
    InternalStudioPatch::new()
    ];
}

pub fn install_patches( data: Rc<RefCell<Binary>>, patches: Vec<String> ) -> Result<(), String> {

    Ok(())
} 