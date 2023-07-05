mod lib;
mod file_map;

use lib::*;
use file_map::get_file_map;
use tauri::{ command, State, Window };
use crate::state::AppState;
use tokio::{ sync::Mutex, task::spawn };
use reqwest::Client;
use std::fs::{ create_dir_all, File, OpenOptions };
use std::io::{ self, Write };
use zip::ZipArchive;
use std::path::PathBuf;
use std::sync::Arc;
use crate::state::InstalledVersion;

#[derive(serde::Deserialize)]
pub struct InstallRequest {
    channel: String,
    version: String
}

#[derive(serde::Serialize, Clone)]
pub struct InstallProgress {
    data: String,
    step: usize,
    total: usize
}

#[command]
pub async fn install_version( 
    window: Window,
    state: State<'_, Arc<Mutex<AppState>>>,
    req: InstallRequest
) -> Result<(), &str> {
    let state = state.inner().clone();

    spawn(async move {  
        let channel = req.channel;
        let version = req.version;

        let mut client = Client::new();
        let manifest = match get_manifest( &mut client, &channel, &version ).await {
            Ok(manifest) => manifest,
            Err(e) => {
                window.emit(
                    "install_version_error", 
                    e.to_string()
                ).unwrap();
                return;
            }
        };

        match create_dir_all( 
            format!( "../versions/{}/{}/", channel, version )
        ) {
            Ok(_) => {},
            Err(e) => {
                window.emit(
                    "install_version_error", 
                    e.to_string()
                ).unwrap();
                return; 
            }
        }

        let step_count = manifest.resources.len() * 2;
        let mut step = 1;

        for resource in manifest.resources {
            window.emit(
                "install_version_progress",
                InstallProgress {
                    data: format!( "Downloading {}", &resource ),
                    step,
                    total: step_count
                }
            ).unwrap();

            step += 1;
            
            let resource_data = match get_resource( &mut client, &channel, &version, &resource ).await {
                Ok(data) => data,
                Err(_) => {
                    window.emit(
                        "install_version_error", 
                        "failed to load resource"
                    ).unwrap();
                    return
                }
            };

            window.emit(
                "install_version_progress",
                InstallProgress {
                    data: format!( "Extracting {}", &resource ),
                    step,
                    total: step_count
                }
            ).unwrap();
            step += 1;
            
            let resource_map = get_file_map();
            let out_dir = resource_map.get( resource.as_str() ).unwrap();
            let out_dir = format!( "../versions/{}/{}{}", &channel, version, out_dir );

            let mut archive = ZipArchive::new( io::Cursor::new( resource_data ) ).unwrap();
            for i in 0..archive.len() {
                let mut file = archive.by_index( i ).unwrap();
                let path = match file.enclosed_name() {
                    Some(path) => path.to_owned(),
                    None => continue
                };

                let out_path = PathBuf::from( &out_dir )
                    .join( &path );

                if let Some(parent) = out_path.parent() {
                    if !parent.exists() {
                        create_dir_all( parent ).unwrap()
                    }
                }

                let mut out_file = File::create( &out_path ).unwrap();
                io::copy( &mut file, &mut out_file ).unwrap();
            }
        }

        
        {
            let mut app_settings = OpenOptions::new()
                .create( true )
                .write( true )
                .open(
                    format!( "../versions/{}/{}/AppSettings.xml", channel, version )
                ).unwrap();
            
            let app_settings_content = include_bytes!( "./install_version/AppSettings.xml" );
            app_settings.write( app_settings_content ).unwrap();
        }

        let mut state = state.lock().await;
        state.installed_versions.push(
            InstalledVersion { 
                channel: channel.clone(), 
                version: version.clone()
            }
        );

        state.write().unwrap();
        window.emit( "installed_version_done", "" ).unwrap();
    });

    Ok(())
}