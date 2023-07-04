#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod state;

use tauri::App;
use std::error::Error;

fn setup_app( app: &mut App ) -> Result<(), Box<dyn Error>> {

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>>{
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![

        ])
        .setup( setup_app )
        .manage( state::load()? )
        .run( tauri::generate_context!() )
        .expect( "error while running tauri application" );

    Ok(())
}
