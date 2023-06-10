mod session;

use session::Session;
use reqwest::Client;
use std::{ thread, time::Duration };

#[tokio::main]
async fn main() {
    let session = Session::new(
        include_bytes!( "../session.dat" )
    );
    
    let client = Client::new();
    for packet in session.packets {
        thread::sleep(
            Duration::from_millis( packet.timing as u64 )
        );

        client
            .post("http://localhost:27773/raknet")
            .body( *packet.data )
            .send()
            .await
            .unwrap();
    }
}