use reqwest::Client;
use std::error::Error;
pub struct Manifest {
    pub resources: Vec<String>
}

impl Manifest {

    pub fn new( manifest: String ) -> Self {
        let lines: Vec<&str> = manifest.lines().collect();

        let mut resources = Vec::new();
        let resource_len = lines.len() / 4;

        for i in 0 .. resource_len {
            resources.push( lines[ (i * 4) + 1 ].into() );
        }

        Manifest { resources } 
    }
}

pub async fn fetch( client: &mut Client, api: &str ) -> Result<String, String> {
    let res = client
        .get( api )
        .send()
        .await;

    let res = match res {
        Ok(res) => res,
        Err(e) => return Err( 
            format!("Failed to fetch version: {}", e)
        )
    };

    if res.status() != 200 {
        return Err( format!("Failed to fetch version") )
    }

    match res.text().await {
        Ok(text) => Ok( text ),
        Err(e) => Err( e.to_string() )
    }
}

pub async fn get_manifest( client: &mut Client, channel: &str, version: &str ) -> Result<Manifest, Box<dyn Error>> {
    let res = fetch(
        client,
        &format!( 
            "https://setup.rbxcdn.com/channel/{}/{}-rbxPkgManifest.txt",
            channel,
            version
        )
    ).await;

    match res {
        Ok(manifest) => Ok( Manifest::new( manifest ) ),
        Err(_) => Err( "Failed to load manifest".into() )
    }
}


pub async fn get_resource( client: &mut Client, channel: &str, version: &str, resource: &str ) -> Result<Vec<u8>, String>  {
    let res = client
        .get( format!("https://setup.rbxcdn.com/channel/{}/{}-{}", channel, version, resource ))
        .send()
        .await;

    let res = match res {
        Ok(res) => res,
        Err(e) => return Err( 
            format!("Failed to get resource: {}", resource)
        )
    };

    match res.bytes().await {
        Ok(resource) => Ok( resource.to_vec() ),
        Err(e) => Err( e.to_string() )
    }
}