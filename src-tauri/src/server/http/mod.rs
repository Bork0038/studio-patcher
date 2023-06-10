use tauri::{ Manager, AppHandle };
use rouille::{ Request, Response };
use std::io::Read;

pub fn handle_connection( app: AppHandle, req: &Request ) {
    if req.method() != "POST" { return };

    let window = match app.get_window( "/http" ) {
        Some(window) => window,
        None => return
    };

    let mut data = match req.data() {
        Some(data) => data,
        None => return
    };
    
    let mut buf = Vec::new();
    data.read_to_end( &mut buf ).unwrap();

    window.emit( "http-data", buf ).unwrap();
}