use tauri::Window;
use tokio::task::spawn;
use reqwest::Client;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(serde::Deserialize)]
pub struct RestoreRequest {
    path: String,
    version: String
}

#[derive(serde::Serialize, Clone)]
pub struct RestoreResult {
    success: bool,
    data: String
}

impl RestoreResult {

    pub fn error<S: Into<String>>( message: S ) -> Self {
        RestoreResult { 
            success: false,
            data: message.into()
        }
    }

    pub fn success() -> Self {
        RestoreResult { 
            success: true, 
            data: String::new()
        }
    }

}


pub async fn restore_binary_internal( req: RestoreRequest ) -> RestoreResult{
    let res = match Client::new()
        .get( format!("https://setup.rbxcdn.com/{}-RobloxStudio.zip", req.version ) )
        .send()
        .await {
            Ok(res) => res,
            Err(e) => return RestoreResult::error( e.to_string() )
        };

    let data = match res.bytes().await {
        Ok(data) => data,
        Err(e) => return RestoreResult::error( e.to_string() )
    };

    let mut file = match OpenOptions::new()
        .write( true )
        .truncate( true )
        .open( &req.path ) {
            Ok(file) => file,
            Err(e) => return RestoreResult::error( e.to_string() )
        };

    match file.write( data.as_ref() ) {
        Ok(_) => RestoreResult::success(),
        Err(e) => RestoreResult::error( e.to_string() )
    }
}

#[tauri::command]
pub fn restore_binary( window: Window, req: RestoreRequest ) {
    spawn(async move {
        let res = restore_binary_internal( req ).await;

        window
            .emit( "restored_binary", res )
            .unwrap()
    });
}