#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// mod commands;
// mod patches;
mod binary;
// mod scanner;

use binary::{ Binary, Section, SectionHeader };

#[tokio::main]
async fn main() {
    unsafe {
        let mut binary = Binary::new(
            include_bytes!( "../../chrome.exe" )
        );

        let new_section = Section::new(
            ".test",
            Vec::with_capacity( 0x1000 )
        );
        binary.add_section( new_section );

        let out = binary.compile();
    }
    // tauri::Builder::default()
    //     .invoke_handler(tauri::generate_handler![
    //         commands::install_patches
    //     ])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
}
