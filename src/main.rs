extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hyper;

mod traits;
mod path_helper;
mod server_config;
mod server_io;
mod responder;
mod mime_mapper;

use hyper::server::Server;
use responder::Responder;

fn main() {
	let config: server_config::ServerConfig = server_io::read_file_json("config.json".to_string()).unwrap();
	let host = config.host.clone();
	let port = config.port.clone();
	let responder = Responder::new(config);

	println!("Starting server on: {}:{}", host, port);

	Server::http(format!("{}:{}", host, port))
		.unwrap()
		.handle(responder)
		.unwrap();
}
