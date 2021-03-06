#![feature(path_file_prefix)]
#[macro_use]
pub mod views;
pub mod data;
pub mod rpc_client;
mod util;

type UserData = std::sync::Arc<std::sync::Mutex<data::Data>>;
