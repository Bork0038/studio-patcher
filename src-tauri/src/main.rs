#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod patches;
mod binary;
mod scanner;

use binary::{ Binary, Section };
use std::{ io::Write, fs::OpenOptions, process::exit };

#[tokio::main]
async fn main() {
    unsafe {
        let out = match Binary::add_section(
            include_bytes!( "../../binary_b.exe" ),
            Section::new(
                ".themes",
                Vec::from( [ 0; 0x1000 ])
            )
        ) {
            Ok(out) => out,
            Err(e) => {
                println!("{}", e.to_string());
                exit(0)
            }
        };
        // let file = File::parse( 
        //     binary_data as &[u8] 
        // ).unwrap(); 

        // let dos_header = ImageDosHeader::parse( binary_data as &[u8] );
        // println!("{:02X}", dos_header.unwrap().e_lfanew.get(LittleEndian));
    
        // let mut binary = Binary::new(
        //     binary_data
        // );

        // // binary.change_file_alignment( 0x400 );

        // let new_section = Section::new(
        //     "vesteria",
        //     Vec::from( [0; 0x1000] )
        // );
        // binary.add_section( new_section );

        // let out = binary.compile();
        let mut o = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create( true )
            .open("./a.exe")
            .unwrap();

        o.write( out.as_ref() );
    }

    // tauri::Builder::default()
    //     .invoke_handler(tauri::generate_handler![
    //         commands::install_patches
    //     ])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
}

