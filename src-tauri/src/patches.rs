mod lib;
pub use lib::*;

mod internal_studio;
mod extended_explorer;

use internal_studio::InternalStudioPatch;
use extended_explorer::ExtendedExplorerPatch;

use std::rc::Rc;
use std::cell::RefCell;
use super::scanner::*;

pub fn get_patches() -> Vec<Patch> {
    vec![
        InternalStudioPatch::new(),
        ExtendedExplorerPatch::new()
    ]
}

pub fn install_patches( data: Rc<RefCell<Vec<u8>>>, patches: Vec<String> ) -> Result<(), String> {
    let patches_list = get_patches();
    
    let mut enabled_patches  = Vec::new();
    for patch in patches_list {
        if patches.contains( &patch.name ) {
            enabled_patches.push( patch );
        }
    }

    for patch in enabled_patches {
        match (patch.patch)( data.clone() ) {
            Ok(_) => {},
            Err(e) => return Err(
                format!(
                    "Failed to apply patch {}:, {}",
                    patch.name,
                    e
                )
            )
        };
    }    

    Ok(())
}

