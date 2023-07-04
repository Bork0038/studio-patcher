use tauri::{ command, State, Window };
use crate::state::AppState;
use tokio::sync::Mutex;

#[command]
pub async fn request_state( window: Window, state: State<'_, Mutex<AppState>> ) -> Result<AppState, String> {
    let state = state.inner().lock().await;

    Ok( state.clone() )
}