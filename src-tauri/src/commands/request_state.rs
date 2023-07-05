use tauri::{ command, State, Window };
use crate::state::AppState;
use tokio::sync::Mutex;
use std::sync::Arc;

#[command]
pub async fn request_state( state: State<'_, Arc<Mutex<AppState>>> ) -> Result<AppState, String> {
    let state = state.inner().lock().await;

    Ok( state.clone() )
}