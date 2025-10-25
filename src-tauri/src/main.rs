// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tiny_http::{Response, Server};

fn main() {

  let server = Server::http("127.0.0.1:8000").unwrap();
    println!("Server running at http://127.0.0.1:8000");

    for request in server.incoming_requests() {
        println!("Received request: {:?}", request);
        let response = Response::from_string("Hello from Rust!");
        request.respond(response).unwrap();
    }

  app_lib::run();
}
