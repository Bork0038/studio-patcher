use tauri::{ Window, Manager, AppHandle };
use tokio::{
    task::{ 
        spawn, 
        JoinHandle 
    },
    sync::{
        Mutex, 
        RwLock
    }
};
use hyper::{ 
    Server,
    service::{ 
        make_service_fn, 
        service_fn 
    }, 
    server::conn::{
        AddrStream, 
        AddrIncoming
    }, 
    Request, 
    Body, 
    Response, 
    Method,
};
use std::{
    net::{ 
        SocketAddrV4,
        Ipv4Addr
    },
    convert::Infallible,
    sync::atomic::{ 
        AtomicBool,
        Ordering
    }
};
use spdlog::prelude::*;
use lazy_static::lazy_static;

lazy_static! {
    static ref started: AtomicBool = AtomicBool::new( false );
}

pub struct HttpServer;

impl HttpServer {

    pub async fn handle_connection( app: AppHandle, label: String, req: Request<Body>) -> Result<Response<Body>, Infallible> {
        if req.method() != &Method::POST {
            return Ok( Response::new("".into()) );
        }

        let bytes = match hyper::body::to_bytes( req.into_body() ).await {
            Ok(bytes) => bytes,
            Err(e) => return Ok( Response::new( "failed".into() ) )
        };
        
        let data: Vec<u8> = bytes
            .iter()
            .map(|d| *d)
            .collect();

        if let Some(window) = app.get_window( &label ) {
            window.emit( "http-data", data ).unwrap();
        }
        
        Ok( Response::new("".into()) )
    }
    
    pub async fn new( window: &Window ) {
        let app = window.app_handle();
        let name = window.label().to_string();

        if !started.load( Ordering::SeqCst ) {
            started.store( true, Ordering::SeqCst );

            spawn( async move {
                let app = app.clone();
                let name = name.clone();
    
                let addr = SocketAddrV4::new(
                    Ipv4Addr::new( 0, 0, 0, 0 ),
                    27773
                );
        
                let make_service = make_service_fn(| _con: &AddrStream | {
                    let app = app.clone();
                    let name = name.clone();
    
                    let service = service_fn( move | req | {
                        HttpServer::handle_connection( app.clone(), name.clone(), req )
                    });
        
                    async move {
                        Ok::<_, Infallible>( service )
                    }
                });
    
                let server = Server::bind( &addr.into() ).serve( make_service );
    
                if let Err(e) = server.await {
                    error!("Http server failed: {}", e);
                }
            });
        }
    }

}