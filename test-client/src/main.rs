mod stream;

use stream::NetworkStream;
use reqwest::Client;
use std::collections::HashMap;

pub struct Request {
    url: String,
    host: String,
    method: String,
    body: Vec<u8>,
    protocol: String,
    headers: HashMap<String, String>
}

pub fn serialize_request( req: Request ) -> NetworkStream {
    let mut stream = NetworkStream::new();

    stream.write_string::<u16>( &req.url );
    stream.write_string::<u16>( &req.host );
    stream.write_string::<u8>( &req.method );
    stream.write_string::<u8>( &req.protocol );

    stream.write_le( req.body.len() as u64 );
    stream.write_bytes( req.body );
    
    stream.write_le( req.headers.len() as u64 );

    for (key, value) in req.headers.iter() {
        stream.write_string::<u64>( key );
        stream.write_string::<u64>( value );
    }

    stream
}

#[tokio::main]
async fn main() {
    let req = Request {
        url: "/index.html".into(),
        host: "example.com".into(),
        method: "GET".into(),
        body: vec![1,2,3,4],
        protocol: "https".into(),
        headers: HashMap::from([
            ("Origin".into(), "https://example.com/a".into())
        ])  
    };

    let mut stream = serialize_request( req );
    let client = Client::new();
    let res = client
        .post("http://localhost:27773")
        .body( stream.get_data() )
        .header( "content-type", "application/octet-stream" )
        .send()
        .await
        .unwrap();

    println!("{}", res.text().await.unwrap());
}
