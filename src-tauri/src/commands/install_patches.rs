use super::super::patches;
use std::fs::OpenOptions;
use std::io::{ Read, Write };

#[derive(serde::Deserialize)]
pub struct PatchRequest {
    path: String,
    patches: Vec<String>
}

#[derive(serde::Serialize)]
pub struct PatchResult {
    success: bool,
    data: String
}

impl PatchResult {

    pub fn error<S: Into<String>>( message: S ) -> Self {
        PatchResult { 
            success: false,
            data: message.into()
        }
    }

    pub fn success() -> Self {
        PatchResult { 
            success: true, 
            data: String::new()
        }
    }

}

#[tauri::command]
pub fn install_patches( patches: PatchRequest ) -> PatchResult {
    let mut data = Box::new( Vec::new() );

    let mut file = match OpenOptions::new()
        .read( true )
        .open( &patches.path ) {
            Ok(file) => file,
            Err(e) => return PatchResult::error( e.to_string() )
        };


    file.read_to_end( &mut data ).unwrap();

    match patches::install_patches( 
        &mut data,  
        patches.patches 
    ) {
        Ok(_) => {},
        Err(e) => return PatchResult::error( e )
    };

    let mut file = match OpenOptions::new()
        .write( true )
        .truncate( true )
        .open( &patches.path ) {
            Ok(file) => file,
            Err(e) => return PatchResult::error( e.to_string() )
        };

    match file.write( data.as_ref() ) {
        Ok(_) => PatchResult::success(),
        Err( e ) => PatchResult::error( e.to_string() )
    }
}