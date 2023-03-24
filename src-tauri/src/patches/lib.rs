use super::{ IDAPat, Scanner };
pub struct Patch {
    pub name: String,
    pub patch: fn( data: &mut Box<Vec<u8>> ) -> Result<(), String>
}

// this is sped
pub enum PatchType {
    ReplacementPatch(ReplacementPatch)
}

impl PatchType {

    pub fn patch( &self, scanner: &Scanner, data: &mut Box<Vec<u8>> ) -> Result<(), String> {
        match self {
            PatchType::ReplacementPatch(p) => p.patch( scanner, data )
        }
    }

}

pub struct OffsetPatch {
    offset: usize,
    bytes: Vec<u8>
}

impl OffsetPatch {
    
    pub fn new( bytes: Vec<u8>, offset: usize,  ) -> Self {
        OffsetPatch { offset, bytes }
    }

}

pub struct ReplacementPatch {
    pattern: IDAPat,
    name: String,
    patches: Vec<OffsetPatch>
}

impl ReplacementPatch {

    pub fn new<S: Into<String>>( pattern: IDAPat, name: S, patches: Vec<OffsetPatch> ) -> PatchType {
        let patch = ReplacementPatch { 
            name: name.into(),
            pattern, 
            patches
        };

        PatchType::ReplacementPatch( patch )
    }

    pub fn patch( &self, scanner: &Scanner, data: &mut Box<Vec<u8>> ) -> Result<(), String> {
        let addr = scanner.scan( &self.pattern )
            .map_or(
                Err( 
                    format!( "Failed to find {}", self.name )
                ),
                | addr | Ok(addr) 
            )?;
        
        for patch in self.patches.iter() {
            let bytes = &patch.bytes;
            let addr  = addr + patch.offset;

            for i in 0..bytes.len() {
                data[ addr + i ] = bytes[ i ];
            }
        }
            
        Ok(())
    }
}