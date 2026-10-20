pub mod application;
#[cfg(not(target_arch = "wasm32"))]
mod connection;
mod input;
#[cfg(not(target_arch = "wasm32"))]
mod replication;
