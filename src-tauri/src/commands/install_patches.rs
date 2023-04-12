use std::fs::OpenOptions;
use std::io::{ Read, Write };
use tokio::task::spawn;
use tauri::Window;
use std::rc::Rc;
use std::cell::RefCell;
use crate::{ patches, Binary };

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
    let mut data = Vec::new();

    {
        let mut file = match OpenOptions::new()
            .read( true )
            .open( &patches.path ) {
                Ok(file) => file,
                Err(e) => return PatchResult::error( e.to_string() )
            };

        if let Err(e) = file.read_to_end( &mut data ) {
            return PatchResult::error( e.to_string() );
        };
    }

    let binary = match Binary::new( data ){
        Ok(binary) => Rc::new( RefCell::new(  binary ) ),
        Err(e) =>  return PatchResult::error( e.to_string() )
    };

    match patches::install_patches( 
        binary.clone(),  
        patches.patches 
    ) {
        Ok(_) => {},
        Err(e) => return PatchResult::error( e.to_string() )
    };

    let mut file = match OpenOptions::new()
        .write( true )
        .truncate( true )
        .open( &patches.path ) {
            Ok(file) => file,
            Err(e) => return PatchResult::error( e.to_string() )
        };

    let data = match binary.borrow_mut().serialize() {
        Ok(data) => data,
        Err(e) => return PatchResult::error( e.to_string() )
    };

    match file.write( data.as_ref() ) {
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