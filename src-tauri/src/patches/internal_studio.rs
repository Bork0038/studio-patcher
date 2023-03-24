use super::{ Patch, IDAPat, Scanner, PatchType, ReplacementPatch, OffsetPatch };
use lazy_static::lazy_static;

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

    pub fn patch( data: &mut Box<Vec<u8>> ) -> Result<(), String> {
        let scanner = Scanner::new( &data );

        for patch in PATCHES.iter() {
            patch.patch( 
                &scanner,
                &mut data.clone() 
            )?;
        }

        Ok(())
    }

}