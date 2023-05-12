use tauri::Window;
use serde::{ Deserialize, Serialize };
use super::super::server;
use lazy_static::lazy_static;
use std::collections::HashMap;
use tokio::task::JoinHandle;
use tokio::task::spawn;

#[derive(Deserialize)]
pub struct ServerRequest {
    pub server_type: String,
    pub server_port: u16
}

#[tauri::command]
pub fn register_server( window: Window, server_info: ServerRequest ) {
    spawn(async move {
        match server_info.server_type.as_str() {
            "http" => server::HttpServer::new( &window ).await,

            _ => return ()
        };
   });
}
