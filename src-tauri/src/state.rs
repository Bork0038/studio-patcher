use std::fs::OpenOptions;
use std::io::{ Read, Write };
use tokio::sync::Mutex;
use serde::{ Deserialize, Serialize };
use std::error::Error;
use std::path::Path;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Clone)]
pub struct InstalledVersion {
    pub channel: String,
    pub version: String
}  

#[derive(Serialize, Deserialize, Clone)]
pub struct AppState {
    pub installed_versions: Vec<InstalledVersion>
}

impl AppState {
    
    pub fn write( &mut self ) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .read( true )
            .write( true )
            .create( true )
            .truncate( true )
            .open( CONFIG_PATH )?;

        file.write_all(
            &*serde_json::to_vec( &self )?
        )?;

        Ok(())
    }

}

const CONFIG_PATH: &str = "./config.json";

pub fn load() -> Result<Arc<Mutex<AppState>>, Box<dyn Error>> {
    let config_path = Path::new( CONFIG_PATH );
    let config_exists = config_path.exists();

    let mut file = OpenOptions::new()
        .read( true )
        .write( true )
        .create( true )
        .open( config_path )?;

    let state = if config_exists {
        let mut data = String::new();
        file.read_to_string( &mut data )?;
        
        serde_json::from_str::<AppState>( &data )?
    } else {
        let state = AppState {
            installed_versions: Vec::new()
        };

        file.write( &serde_json::to_vec( &state )? )?;
        state
    };

    Ok(
        Arc::new( Mutex::new( state ) )
    )
}