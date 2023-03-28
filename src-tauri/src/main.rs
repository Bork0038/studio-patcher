#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod patches;
// mod binary;
mod scanner;

// use binary::{ Binary, Section, SectionHeader };
// use std::{ io::Write, fs::OpenOptions };

#[tokio::main]
async fn main() {
    // unsafe {
    //     let binary_data = include_bytes!( "../../binary.exe" );
    //     let mut binary = Binary::new(
    //         binary_data
    //     );

    //     let new_section = Section::new(
    //         "vesteria",
    //         Vec::with_capacity( 0x1000 )
    //     );
    //     binary.add_section( new_section );

        // let out = binary.compile();
        // for i in 0..out.len() {
        //     let byte = out[i];

        //     if byte != binary_data[i] {
        //         println!("{} {:02X} = {:02X}", i, byte, binary_data[i])
        //     }
        // }

        
        // let out = binary.compile();
        // let mut o = OpenOptions::new()
        //     .write(true)
        //     .truncate(true)
        //     .create( true )
        //     .open("./a.exe")
        //     .unwrap();

        // o.write( out.as_ref() );
    // }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::install_patches
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
