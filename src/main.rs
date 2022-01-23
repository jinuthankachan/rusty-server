#![allow(dead_code)]

mod handler;
mod http;
mod server;

use std::env;

use server::Server;

fn main() {
    let default_path = format!("{}/public",env!("CARGO_MANIFEST_DIR").to_string());
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("::Starting server::");
    let srv = Server::new("127.0.0.1:8080".to_string());
    srv.run(handler::WebsiteHandler::new(public_path));
}
