mod lib;
pub use lib::*;

mod internal_studio;
mod extended_explorer;

use internal_studio::InternalStudioPatch;
use extended_explorer::ExtendedExplorerPatch;

use crate::Binary;
use std::{ rc::Rc, cell::RefCell };
use lazy_static::lazy_static;
use std::error::Error;

lazy_static! {
    static ref patch_list: Vec<Patch> = vec![
        InternalStudioPatch::new(),
        ExtendedExplorerPatch::new()
    ];
}

pub fn install_patches( data: Rc<RefCell<Binary>>, patches: Vec<String> ) -> Result<(), Box<dyn Error>> {
    let mut enabled_patches  = Vec::new();
    for patch in patch_list.iter() {
        if patches.contains( &patch.name ) {
            enabled_patches.push( patch );
        }
    }

    for patch in enabled_patches {
        ( patch.patch )( data.clone() )?;
    }    


    Ok(())
} 