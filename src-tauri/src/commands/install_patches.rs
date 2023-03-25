use super::super::patches;
use std::fs::OpenOptions;
use std::io::{ Read, Write };
use tokio::task::spawn;
use tauri::Window;

#[derive(serde::Deserialize)]
pub struct PatchRequest {
    path: String,
    patches: Vec<String>
}

#[derive(serde::Serialize, Clone)]
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

pub fn install_patches_internal( patches: PatchRequest ) -> PatchResult {
    let mut cell: Vec<u8> = Vec::new(); 

    let mut file = match OpenOptions::new()
        .read( true )
        .open( &patches.path ) {
            Ok(file) => file,
            Err(e) => return PatchResult::error( e.to_string() )
        };

    file.read_to_end( &mut cell ).unwrap();
    
    match patches::install_patches( 
        &mut cell[0..],  
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
    
    match file.write( cell.as_ref() ) {
        Ok(_) => PatchResult::success(),
        Err( e ) => PatchResult::error( e.to_string() )
    }
}

#[tauri::command]
pub fn install_patches( window: Window, patches: PatchRequest ) {
    spawn(async move {
        let res = install_patches_internal( patches );

        window
            .emit( "installed_patches", res )
            .unwrap();
    });
}