use std::fs::{copy, create_dir_all};

fn main() {
	create_dir_all("./dlls").unwrap();
	copy("../cpp/build/http/Debug/http.dll", "./dlls/http.dll").unwrap();
	copy("../cpp/build/loader/Debug/loader.dll", "./dlls/loader.dll").unwrap();

  	tauri_build::build()
}
