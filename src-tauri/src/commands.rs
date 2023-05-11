mod install_patches;
mod register_server;
mod restore_binary;

pub use install_patches::{ __cmd__install_patches, install_patches };
pub use restore_binary::{ __cmd__restore_binary, restore_binary };
pub use register_server::{ __cmd__register_server, register_server };