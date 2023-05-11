use tauri::Window;
use serde::{ Deserialize, Serialize };
use super::super::server;

#[derive(Deserialize)]
pub struct ServerRequest {
    pub server_type: String,
    pub server_port: u16
}

#[tauri::command]
pub fn register_server( window: Window, server_info: ServerRequest ) {
    match server_info.server_type.as_str() {
        "http" => server::HttpServer::new( &window ),
        _ => {}
    }
}
