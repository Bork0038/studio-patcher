use tauri::{ Window, Manager };
use serde::{ Deserialize, Serialize };
use std::time::Duration;
use std::thread::sleep;
use tokio::task::spawn;
use super::super::server::raknet::packet::lib::packets::lib::NetworkSchema;

#[derive(Deserialize, Serialize)]
pub struct SchemaRequest {
    id: String,
    data: NetworkSchema
}

#[tauri::command]
pub fn register_schema( window: Window, schema: SchemaRequest ) {
    spawn(async move {
        let app_handle = window.app_handle();
        let window_name = format!( "/schema{}", schema.id );

        let sleep_time = Duration::from_millis( 500 );
        sleep( sleep_time );
        
        if let Some(window) = app_handle.get_window( &window_name ) {
            if let Ok(()) = window.emit( "init-schema", &schema.data ) {
                println!("sent");
            }
        }
    });
}