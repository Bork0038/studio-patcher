mod http;
mod raknet;

use tauri::App;
use tokio::task::spawn;
use std::sync::atomic::{
    AtomicBool,
    Ordering
};
use lazy_static::lazy_static;
use rouille::Response;

lazy_static! {
    static ref started: AtomicBool = AtomicBool::new( false );
}

pub fn init_servers( app: &mut App ) {
    if !started.load( Ordering::SeqCst ) {
        started.store( true, Ordering::SeqCst );

        let app = app.handle();
        spawn(async move {
            let app = app.clone();

            rouille::start_server( "0.0.0.0:27773", move | req | {
                let app = app.clone();

                match req.url().as_str() {
                    "/http" => http::handle_connection( app, req ),
                    "/raknet" => raknet::handle_connection( app, req ),
                    _ => {}
                };

                Response::text("")
            });
        });
    }
}