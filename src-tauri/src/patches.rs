mod internal_studio;
mod extended_explorer;

use internal_studio::InternalStudioPatch;
use extended_explorer::ExtendedExplorerPatch;

use std::collections::HashMap;
use super::scanner::*;

pub struct Patch {
    name: String,
    patch: fn( data: &mut Box<Vec<u8>> ) -> Result<(), String>
}

pub fn get_patches() -> Vec<Patch> {
    vec![
        InternalStudioPatch::new(),
        ExtendedExplorerPatch::new()
    ]
}

pub fn patch_segment( data: &mut Box<Vec<u8>>, patch: Vec<u8>, addr: usize ) {
    for i in 0..patch.len() {
        data[ addr + i ] = patch[ i ];
    }
}


pub fn install_patches( data: &mut  Box<Vec<u8>>, patches: Vec<String> ) -> Result<(), String> {
    let patches_list = get_patches();
    
    let mut enabled_patches  = Vec::new();
    for patch in patches_list {
        if patches.contains( &patch.name ) {
            enabled_patches.push( patch );
        }
    }

    for patch in enabled_patches {
        match (patch.patch)( data ) {
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

