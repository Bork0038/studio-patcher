use super::{ Patch, PatchType, ReplacementPatch, OffsetPatch};
use crate::binary::{ IDAPat, Binary };
use std::rc::Rc;
use std::cell::RefCell;
use std::error::Error;
use std::fs::{ copy };
use std::path::PathBuf;

pub struct HttpSpyPatch;

impl HttpSpyPatch  {

    pub fn new() -> Patch {
        Patch {
            name: "http-spy".into(),
            patch: HttpSpyPatch ::patch
        }
    }

    pub fn patch( binary: Rc<RefCell<Binary>>, file_path: String ) -> Result<(), Box<dyn Error>> {
        let bin = binary.borrow_mut();
        
        let mut path = PathBuf::from( file_path );
        path.pop();

        path.push("WebView2LoaderOld.dll");
        if !path.exists() {
            let new_path = path.clone();

            path.pop();
            path.push("WebView2Loader.dll");

            copy(&path, new_path)?;
            copy("./dlls/loader.dll", &path)?;

            path.pop();
        }

        // check if webview2loader has already been swapped
        
        // rename WebView2Loader

        path.push("http.dll");
        copy("./dlls/http.dll", path)?;

        Ok(())
    }

}
