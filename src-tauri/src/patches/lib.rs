use crate::binary::{ Binary, IDAPat, CodePat };
use std::rc::Rc;
use std::cell::RefCell;
use std::error::Error;

pub struct Patch {
    pub name: String,
    pub patch: fn( data: Rc<RefCell<Binary>> ) -> Result<(), Box<dyn Error>>
}

pub enum PatchType {
    ReplacementPatch(ReplacementPatch)
}

pub struct OffsetPatch {
    offset: usize,
    bytes: Vec<u8>
}

impl OffsetPatch {
    
    pub fn new( bytes: Vec<u8>, offset: usize ) -> Self {
        OffsetPatch { offset, bytes }
    }

}

pub struct ReplacementPatch {
    pattern: IDAPat,
    section: String,
    name: String,
    patches: Vec<OffsetPatch>
}

impl ReplacementPatch {

    pub fn new<Name, Section>( pattern: IDAPat, name: Name, section: Section, patches: Vec<OffsetPatch> ) -> PatchType 
    where Name: Into<String>, Section: Into<String>
    {
        let patch = ReplacementPatch {
            name: name.into(),
            section: section.into(),
            pattern,
            patches
        };

        PatchType::ReplacementPatch( patch )
    }

    pub fn patch( &self, binary: Rc<RefCell<Binary>> ) -> Result<(), Box<dyn Error>> {
        let mut binary = binary.borrow_mut();

        let addr = binary.scan( &self.pattern, Some( &self.section ) )
            .map_or(
                Err(
                    format!("Failed to find {}", self.name )
                ),
                | addr | Ok( addr )
            )?;

        let section = binary
            .get_section_by_name( &self.section )
            .map_or(
                Err(
                    format!( "Failed to find section: {}", &self.section )
                ),
                | section | Ok( section )
            )?;

        for patch in self.patches.iter() {
            let bytes = &patch.bytes;
            let addr = addr + patch.offset;

            for i in 0..bytes.len() {
                section.data[ addr + i ] = bytes[ i ];
            }
        }

        Ok(())
    }

}