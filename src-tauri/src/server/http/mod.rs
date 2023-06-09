use tauri::{ Manager, AppHandle };
use rouille::{ Request, Response };
use std::io::Read;

pub fn handle_connection( app: AppHandle, req: &Request ) {
    if req.method() != "POST" { return };

    if let Some(window) = app.get_window( "http" ) {
        if let Some(mut data) = req.data() {
            let mut buf = Vec::new();
            data.read_to_end( &mut buf ).unwrap();

            window.emit( "http-data", buf ).unwrap();
        }
    } 
}